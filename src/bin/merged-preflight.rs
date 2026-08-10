use std::env;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const MODEL_SHA256: &str = "14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7";
const TOKENIZER_SHA256: &str = "79e3e522635f3171300913bb421464a87de6222182a0570b9b2ccba2a964b2b4";
const NIODOO_SHA256: &str = "3d04277456ea8ec8d998c9a7fb94b38d2613df65b8a1ec6c8727fe67f040b920";

struct Config {
    map_root: PathBuf,
    source_root: PathBuf,
    product_root: PathBuf,
    llama_cli: PathBuf,
    niodoo_bin: PathBuf,
    model: PathBuf,
    tokenizer: PathBuf,
    out: PathBuf,
    live_resume: Option<PathBuf>,
    live_cold_store_only: bool,
}

struct Captured {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    success: bool,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Mode {
    Preflight,
    Live,
}

fn mode() -> Result<Mode, String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => Ok(Mode::Preflight),
        Some("--live") if args.next().is_none() => Ok(Mode::Live),
        _ => Err("usage: merged-preflight [--live]".to_string()),
    }
}

fn env_path(name: &str, default: impl Into<PathBuf>) -> PathBuf {
    env::var_os(name)
        .map(PathBuf::from)
        .unwrap_or_else(|| default.into())
}

fn config(mode: Mode) -> Result<Config, String> {
    let map_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/home/ruffianl"));
    let source_root = env_path(
        "MERGED_PREFLIGHT_SOURCE_ROOT",
        "/home/ruffianl/projects/niodoo-arc-rehit",
    );
    let product_root = env_path(
        "MERGED_PREFLIGHT_PRODUCT_ROOT",
        "/home/ruffianl/Hub/Projects/niodoo/niodoo-live",
    );
    let run_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("clock error: {error}"))?
        .as_nanos();
    let prefix = if mode == Mode::Live {
        "merged-live"
    } else {
        "merged-preflight"
    };
    let out = env_path(
        "MERGED_PREFLIGHT_OUT",
        map_root.join(format!("runs/{prefix}-{run_id}-{}", std::process::id())),
    );
    Ok(Config {
        llama_cli: env_path(
            "MERGED_PREFLIGHT_LLAMA_CLI",
            home.join(".local/bin/llama-cli"),
        ),
        niodoo_bin: env_path(
            "MERGED_PREFLIGHT_NIODOO_BIN",
            map_root.join(".one-shot/target-niodoo-tools/release/niodoo"),
        ),
        model: env_path(
            "MERGED_PREFLIGHT_MODEL",
            product_root.join("model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf"),
        ),
        tokenizer: env_path(
            "MERGED_PREFLIGHT_TOKENIZER",
            source_root.join("model/tokenizer.json"),
        ),
        map_root,
        source_root,
        product_root,
        out,
        live_resume: env::var_os("MERGED_LIVE_RESUME").map(PathBuf::from),
        live_cold_store_only: env::var("MERGED_LIVE_COLD_STORE_ONLY")
            .map(|value| matches!(value.trim(), "1" | "true" | "on"))
            .unwrap_or(false),
    })
}

fn sha256(path: &Path) -> Result<String, String> {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .map_err(|error| format!("could not run sha256sum: {error}"))?;
    if !output.status.success() {
        return Err(format!("sha256sum failed for {}", path.display()));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("invalid sha256sum output: {error}"))?
        .split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| format!("empty sha256sum output for {}", path.display()))
}

fn require_sha(path: &Path, expected: &str, label: &str) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("{label} missing: {}", path.display()));
    }
    let actual = sha256(path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{label} hash mismatch: expected {expected}, got {actual}"
        ))
    }
}

fn isolated_command(program: &Path) -> Command {
    let inherited: Vec<(OsString, OsString)> = env::vars_os()
        .filter(|(key, _)| !key.to_string_lossy().starts_with("NIODOO_"))
        .collect();
    let mut command = Command::new(program);
    command.env_clear().envs(inherited);
    command
}

fn tee_reader<R: Read + Send + 'static>(
    mut reader: R,
    path: PathBuf,
    to_stderr: bool,
) -> thread::JoinHandle<Result<Vec<u8>, String>> {
    thread::spawn(move || {
        let mut file = File::create(&path)
            .map_err(|error| format!("could not create {}: {error}", path.display()))?;
        let mut complete = Vec::new();
        let mut buffer = [0u8; 16 * 1024];
        loop {
            let count = reader
                .read(&mut buffer)
                .map_err(|error| format!("could not read child stream: {error}"))?;
            if count == 0 {
                break;
            }
            let bytes = &buffer[..count];
            file.write_all(bytes)
                .map_err(|error| format!("could not write {}: {error}", path.display()))?;
            if to_stderr {
                let mut stream = std::io::stderr().lock();
                stream.write_all(bytes).and_then(|_| stream.flush())
            } else {
                let mut stream = std::io::stdout().lock();
                stream.write_all(bytes).and_then(|_| stream.flush())
            }
            .map_err(|error| format!("could not mirror child stream: {error}"))?;
            complete.extend_from_slice(bytes);
        }
        Ok(complete)
    })
}

fn capture(command: &mut Command, dir: &Path, label: &str, stem: &str) -> Result<Captured, String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("could not start {label}: {error}"))?;
    println!("\n--- {label}: process {} ---", child.id());
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| format!("{label} stdout pipe missing"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| format!("{label} stderr pipe missing"))?;
    let stdout_handle = tee_reader(stdout, dir.join(format!("{stem}.stdout.txt")), false);
    let stderr_handle = tee_reader(stderr, dir.join(format!("{stem}.stderr.txt")), true);
    let status = child
        .wait()
        .map_err(|error| format!("could not wait for {label}: {error}"))?;
    let stdout = stdout_handle
        .join()
        .map_err(|_| format!("{label} stdout thread panicked"))??;
    let stderr = stderr_handle
        .join()
        .map_err(|_| format!("{label} stderr thread panicked"))??;
    println!("\n--- {label}: exited {status} ---");
    Ok(Captured {
        stdout,
        stderr,
        success: status.success(),
    })
}

fn tee_stdin(mut child_stdin: std::process::ChildStdin, path: PathBuf) {
    thread::spawn(move || {
        let Ok(mut file) = File::create(&path) else {
            return;
        };
        let stdin = std::io::stdin();
        let mut input = stdin.lock();
        let mut buffer = [0u8; 4096];
        loop {
            let Ok(count) = input.read(&mut buffer) else {
                break;
            };
            if count == 0 {
                break;
            }
            let bytes = &buffer[..count];
            if file.write_all(bytes).and_then(|_| file.flush()).is_err() {
                break;
            }
            if child_stdin
                .write_all(bytes)
                .and_then(|_| child_stdin.flush())
                .is_err()
            {
                break;
            }
        }
    });
}

fn capture_interactive(
    command: &mut Command,
    dir: &Path,
    label: &str,
    stem: &str,
) -> Result<Captured, String> {
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("could not start {label}: {error}"))?;
    println!("\n--- {label}: process {} ---", child.id());
    let child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| format!("{label} stdin pipe missing"))?;
    tee_stdin(child_stdin, dir.join(format!("{stem}.stdin.txt")));
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| format!("{label} stdout pipe missing"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| format!("{label} stderr pipe missing"))?;
    let stdout_handle = tee_reader(stdout, dir.join(format!("{stem}.stdout.txt")), false);
    let stderr_handle = tee_reader(stderr, dir.join(format!("{stem}.stderr.txt")), true);
    let status = child
        .wait()
        .map_err(|error| format!("could not wait for {label}: {error}"))?;
    let stdout = stdout_handle
        .join()
        .map_err(|_| format!("{label} stdout thread panicked"))??;
    let stderr = stderr_handle
        .join()
        .map_err(|_| format!("{label} stderr thread panicked"))??;
    println!("\n--- {label}: exited {status} ---");
    Ok(Captured {
        stdout,
        stderr,
        success: status.success(),
    })
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("could not create {}: {error}", destination.display()))?;
    for entry in fs::read_dir(source)
        .map_err(|error| format!("could not read {}: {error}", source.display()))?
    {
        let entry = entry.map_err(|error| format!("could not read directory entry: {error}"))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("could not inspect {}: {error}", entry.path().display()))?;
        let target = destination.join(entry.file_name());
        if file_type.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), &target).map_err(|error| {
                format!(
                    "could not copy {} to {}: {error}",
                    entry.path().display(),
                    target.display()
                )
            })?;
        } else {
            return Err(format!(
                "resume workspace contains unsupported link or special file: {}",
                entry.path().display()
            ));
        }
    }
    Ok(())
}

fn resume_session_dir(path: &Path) -> Result<PathBuf, String> {
    let direct = path.join("isolated-remember-store.jsonl");
    if direct.is_file() {
        return Ok(path.to_path_buf());
    }
    let nested = path.join("niodoo-live-session");
    if nested.join("isolated-remember-store.jsonl").is_file() {
        return Ok(nested);
    }
    Err(format!(
        "resume source has no isolated Remember store: {}",
        path.display()
    ))
}

fn prepare_output(config: &Config) -> Result<(), String> {
    let parent = config
        .out
        .parent()
        .ok_or_else(|| format!("output has no parent: {}", config.out.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::create_dir(&config.out).map_err(|error| {
        format!(
            "refusing non-new output directory {}: {error}",
            config.out.display()
        )
    })
}

fn extract_prompt(config: &Config) -> Result<(Vec<u8>, String), String> {
    let dir = config.out.join("prompt-receipt");
    fs::create_dir(&dir).map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let prompt_home = dir.join("isolated-home");
    fs::create_dir(&prompt_home)
        .map_err(|error| format!("could not create {}: {error}", prompt_home.display()))?;
    let mut command = isolated_command(&config.niodoo_bin);
    command
        .current_dir(&config.source_root)
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env("NIODOO_SPACE_HOME", &prompt_home)
        .env("NIODOO_VAULT_URL", "off")
        .arg("--model-path")
        .arg(&config.model)
        .args([
            "--system-prompt-mode",
            "free",
            "--workspace-tools",
            "true",
            "--print-resolved-system-prompt",
        ]);
    let output = capture(&mut command, &dir, "exact Niodoo prompt receipt", "process")?;
    if !output.success || !output.stderr.is_empty() {
        return Err("prompt receipt failed or wrote unexpected stderr".to_string());
    }
    let prompt = String::from_utf8(output.stdout.clone())
        .map_err(|error| format!("resolved system prompt is not UTF-8: {error}"))?;
    for required in [
        "CONTROL CHANNEL",
        "# Tools",
        r#""name":"list""#,
        r#""name":"read""#,
        r#""name":"write""#,
        "Agent home is an isolated writable workspace",
    ] {
        if !prompt.contains(required) {
            return Err(format!("resolved prompt missing {required:?}"));
        }
    }
    if (prompt.contains(r#""name":"read_file""#) || prompt.contains(r#""name": "read_file""#))
        || prompt.contains(prompt_home.to_string_lossy().as_ref())
    {
        return Err(
            "resolved prompt exposes a false tool name or process-specific home".to_string(),
        );
    }
    let prompt_path = dir.join("resolved-system-prompt.txt");
    fs::write(&prompt_path, &output.stdout)
        .map_err(|error| format!("could not write {}: {error}", prompt_path.display()))?;
    let prompt_sha = sha256(&prompt_path)?;
    fs::write(
        dir.join("RECEIPT.txt"),
        format!(
            "source=Niodoo --print-resolved-system-prompt\nbytes={}\nsha256={prompt_sha}\nstderr_bytes=0\nstatus=EXACT\n",
            output.stdout.len()
        ),
    )
    .map_err(|error| format!("could not write prompt receipt: {error}"))?;
    Ok((output.stdout, prompt_sha))
}

fn run_llama_control(config: &Config, prompt: &[u8], prompt_sha: &str) -> Result<(), String> {
    let dir = config.out.join("llama-control");
    fs::create_dir(&dir).map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let system_prompt = String::from_utf8(prompt.to_vec())
        .map_err(|error| format!("system prompt is not UTF-8: {error}"))?;
    let user_prompt_path = config
        .map_root
        .join("coordinates/vanilla/original/trap.txt");
    let user_prompt = fs::read_to_string(&user_prompt_path)
        .map_err(|error| format!("could not read {}: {error}", user_prompt_path.display()))?;
    fs::write(dir.join("system-prompt.txt"), prompt)
        .map_err(|error| format!("could not write llama prompt copy: {error}"))?;
    fs::write(dir.join("user-prompt.txt"), &user_prompt)
        .map_err(|error| format!("could not write llama user prompt copy: {error}"))?;
    let generated = dir.join("complete-conversation.txt");
    let mut command = isolated_command(&config.llama_cli);
    command
        .current_dir(&config.map_root)
        .arg("-m")
        .arg(&config.model)
        .arg("-p")
        .arg(&user_prompt)
        .arg("--system-prompt")
        .arg(&system_prompt)
        .args(["-n", "512", "--temp", "0.0", "--seed", "42"])
        .args([
            "-c",
            "4096",
            "-ngl",
            "99",
            "-st",
            "--jinja",
            "--no-display-prompt",
            "--simple-io",
        ])
        .arg("--output-file")
        .arg(&generated);
    let output = capture(
        &mut command,
        &dir,
        "llama.cpp matched full-tools control",
        "process",
    )?;
    if !output.success || !generated.is_file() {
        return Err("matched llama.cpp control failed".to_string());
    }
    fs::write(
        dir.join("RECEIPT.txt"),
        format!(
            "system_prompt_source=../prompt-receipt/resolved-system-prompt.txt\nsystem_prompt_sha256={prompt_sha}\nuser_prompt=coordinates/vanilla/original/trap.txt\ncorrect_answer=[5, 4, 3, 2, 1, 5]\nstatus=CAPTURED_UNSCORED_CONTROL\n"
        ),
    )
    .map_err(|error| format!("could not write llama receipt: {error}"))?;
    Ok(())
}

fn run_tool_smoke(config: &Config, prompt_sha: &str) -> Result<(), String> {
    let dir = config.out.join("niodoo-tool-smoke");
    fs::create_dir(&dir).map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let tool_home = dir.join("isolated-tool-home");
    fs::create_dir(&tool_home)
        .map_err(|error| format!("could not create {}: {error}", tool_home.display()))?;
    let store = dir.join("isolated-remember-store.jsonl");
    fs::write(&store, b"").map_err(|error| format!("could not create store: {error}"))?;
    let session = dir.join("session.txt");
    fs::write(
        &session,
        b"Use your workspace tools to write exactly TOOL_SMOKE_OK to tools/roundtrip.txt, then read that file back, then tell me exactly what you read.\n",
    )
    .map_err(|error| format!("could not create tool-smoke session: {error}"))?;
    let mut command = isolated_command(&config.niodoo_bin);
    command
        .current_dir(&config.source_root)
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env("NIODOO_SPACE_HOME", &tool_home)
        .env("NIODOO_VAULT_URL", "off")
        .arg("--model-path")
        .arg(&config.model)
        .args(["--model-size", "8b", "--model-arch", "auto"])
        .arg("--tokenizer-path")
        .arg(&config.tokenizer)
        .args(["--chat-template", "auto", "--system-prompt-mode", "free"])
        .args(["--stdout-profile", "chat"])
        .args(["--output-contract-mode", "off", "--context-length", "8192"])
        .args(["--max-steps", "512", "--temperature", "0.0"])
        .args(["--theta-override", "1.5", "--physics-blend", "0.9"])
        .args(["--physics-start-layer", "16", "--physics-end-layer", "33"])
        .args(["--ablate-periodic-controller", "--ablate-live-motifs"])
        .args(["--workspace-tools", "true", "--max-tool-rounds", "6"])
        .arg("--remember-store")
        .arg(&store)
        .arg("--session-script")
        .arg(&session)
        .arg("--telemetry-out")
        .arg(dir.join("telemetry.jsonl"))
        .args(["--telemetry-profile", "minimal"]);
    let output = capture(
        &mut command,
        &dir,
        "Niodoo isolated live tool smoke",
        "complete",
    )?;
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    let roundtrip = tool_home.join("tools/roundtrip.txt");
    let payload = fs::read(&roundtrip).unwrap_or_default();
    let prompt_receipt = format!(r#""system_prompt_sha256":"{prompt_sha}""#);
    let prompt_receipt_spaced = format!(r#""system_prompt_sha256": "{prompt_sha}""#);
    let used_write = combined.contains("[TOOLS]") && combined.contains("write");
    let used_read = combined.contains("[TOOLS]") && combined.contains("read");
    let no_parse_failure = !combined.contains("parse_error")
        && !combined.contains("unknown function")
        && !combined.contains("[REMEMBER-VAULT] client ready")
        && !combined.contains("saved self-memory");
    let prompt_matched =
        combined.contains(&prompt_receipt) || combined.contains(&prompt_receipt_spaced);
    let store_text = fs::read_to_string(&store).unwrap_or_default();
    let store_entries = store_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let passed = output.success
        && payload == b"TOOL_SMOKE_OK"
        && used_write
        && used_read
        && no_parse_failure
        && prompt_matched;
    let gate = format!(
        "process_success={}\nroundtrip_exact={}\nwrite_receipt={}\nread_receipt={}\nprompt_sha256_matched={}\nqdrant=OFF\nremember_store_isolated=true\nremember_store_entries={}\nparse_and_vault_clean={}\ntool_home={}\nremember_store={}\ncorrect_payload=TOOL_SMOKE_OK\nstatus={}\n",
        output.success,
        payload == b"TOOL_SMOKE_OK",
        used_write,
        used_read,
        prompt_matched,
        store_entries,
        no_parse_failure,
        tool_home.display(),
        store.display(),
        if passed { "PASS" } else { "FAIL" },
    );
    fs::write(dir.join("GATE.txt"), &gate)
        .map_err(|error| format!("could not write tool gate: {error}"))?;
    println!("\n{gate}");
    if passed {
        Ok(())
    } else {
        Err("isolated Niodoo tool smoke did not pass; inspect its complete raw streams".to_string())
    }
}

fn run_live_session(config: &Config, prompt_sha: &str) -> Result<(), String> {
    let dir = config.out.join("niodoo-live-session");
    fs::create_dir(&dir).map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let tool_home = dir.join("isolated-tool-home");
    fs::create_dir(&tool_home)
        .map_err(|error| format!("could not create {}: {error}", tool_home.display()))?;
    let store = dir.join("isolated-remember-store.jsonl");
    let compact_resume = dir.join("compact-resume-state.json");
    let mut resume_compact = None;
    let resume_source = if let Some(requested) = config.live_resume.as_deref() {
        let source = resume_session_dir(requested)?;
        fs::copy(source.join("isolated-remember-store.jsonl"), &store)
            .map_err(|error| format!("could not carry Remember store forward: {error}"))?;
        if !config.live_cold_store_only {
            let source_home = source.join("isolated-tool-home");
            if source_home.is_dir() {
                copy_tree(&source_home, &tool_home)?;
            }
            let source_compact = source.join("compact-resume-state.json");
            if source_compact.is_file() {
                resume_compact = Some(source_compact);
            }
        }
        Some(source)
    } else {
        fs::write(&store, b"").map_err(|error| format!("could not create store: {error}"))?;
        None
    };

    let mut command = isolated_command(&config.niodoo_bin);
    command
        .current_dir(&config.source_root)
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env("NIODOO_SPACE_HOME", &tool_home)
        .env("NIODOO_VAULT_URL", "off")
        .arg("--model-path")
        .arg(&config.model)
        .args(["--model-size", "8b", "--model-arch", "auto"])
        .arg("--tokenizer-path")
        .arg(&config.tokenizer)
        .args(["--chat-template", "auto", "--system-prompt-mode", "free"])
        .args(["--runtime-profile", "legacy-public"])
        .args(["--cache-backend", "legacy-concat"])
        .args(["--session-mode", "continuous"])
        .args(["--stdout-profile", "chat"])
        .args(["--output-contract-mode", "off", "--context-length", "8192"])
        .args(["--max-steps", "768", "--temperature", "0.0"])
        .args(["--theta-override", "1.5", "--physics-blend", "0.9"])
        .args(["--physics-start-layer", "16", "--physics-end-layer", "33"])
        .args(["--ablate-periodic-controller", "--ablate-live-motifs"])
        .args(["--workspace-tools", "true", "--max-tool-rounds", "6"])
        .args(["--chat-repl", "--lock-stop-policy", "off"])
        .arg("--remember-store")
        .arg(&store)
        .arg("--compact-resume-state-save-file")
        .arg(&compact_resume)
        .arg("--telemetry-out")
        .arg(dir.join("telemetry.jsonl"))
        .args(["--telemetry-profile", "minimal"]);
    if let Some(path) = resume_compact.as_deref() {
        command.arg("--compact-resume-state-load-file").arg(path);
    }

    println!("\n=== NATURAL SESSION STARTS HERE ===");
    println!(
        "Nothing has been scripted. Type naturally; use /exit when you choose to end the room.\n"
    );
    let output = capture_interactive(
        &mut command,
        &dir,
        "Niodoo user-operated natural session",
        "complete",
    )?;
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    let prompt_receipt = format!(r#""system_prompt_sha256":"{prompt_sha}""#);
    let prompt_matched = combined.contains(&prompt_receipt);
    let vault_off = !combined.contains("[REMEMBER-VAULT] client ready")
        && !combined.contains("saved self-memory");
    let store_text = fs::read_to_string(&store).unwrap_or_default();
    let entries = store_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let receipt = format!(
        "process_success={}\nprompt_sha256_matched={}\nprompt_sha256={}\nqdrant={}\nresume_source={}\nresume_scope={}\nuser_input_capture={}\ncompact_resume_state={}\nremember_store_entries={}\ntool_home={}\nremember_store={}\nstatus={}\n",
        output.success,
        prompt_matched,
        prompt_sha,
        if vault_off { "OFF" } else { "UNEXPECTED_ACTIVITY" },
        resume_source
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "NONE_FRESH_ROOM".to_string()),
        if resume_source.is_some() && config.live_cold_store_only {
            "DURABLE_REMEMBER_STORE_ONLY"
        } else if resume_source.is_some() {
            "REMEMBER_WORKSPACE_AND_COMPACT_RESUME"
        } else {
            "FRESH_EMPTY"
        },
        dir.join("complete.stdin.txt").display(),
        compact_resume.display(),
        entries,
        tool_home.display(),
        store.display(),
        if output.success && prompt_matched && vault_off {
            "SESSION_CAPTURED"
        } else {
            "CAPTURE_FAILED"
        },
    );
    fs::write(dir.join("RECEIPT.txt"), &receipt)
        .map_err(|error| format!("could not write live receipt: {error}"))?;
    println!("\n{receipt}");
    if output.success && prompt_matched && vault_off {
        Ok(())
    } else {
        Err("natural session capture failed its infrastructure receipt".to_string())
    }
}

fn preflight(config: &Config) -> Result<(), String> {
    if env::consts::OS != "linux" || env::consts::ARCH != "aarch64" {
        return Err(format!(
            "exact lane requires Linux aarch64; found {} {}",
            env::consts::OS,
            env::consts::ARCH
        ));
    }
    require_sha(&config.model, MODEL_SHA256, "model")?;
    require_sha(&config.tokenizer, TOKENIZER_SHA256, "tokenizer")?;
    require_sha(
        &config.niodoo_bin,
        NIODOO_SHA256,
        "new-coordinate Niodoo binary",
    )?;
    if !config.llama_cli.is_file() {
        return Err(format!(
            "llama.cpp binary missing: {}",
            config.llama_cli.display()
        ));
    }
    prepare_output(config)
}

fn write_run_map(config: &Config, status: &str) {
    let _ = fs::write(
        config.out.join("RUN_MAP.md"),
        format!(
            "# Merged organic-route preflight\n\n## Status\n\n{status}\n\nThis is an engineering preflight, not an earned organic run and not a new flag. The complete, unedited subprocess streams are retained under this directory. Qdrant was forced off, and prompt inspection and the live tool smoke used separate isolated homes.\n"
        ),
    );
}

fn real_main(config: &Config, mode: Mode) -> Result<(), String> {
    println!(
        "{}",
        if mode == Mode::Live {
            "MERGED USER-OPERATED NATURAL SESSION — NOT YET A FLAG RUN"
        } else {
            "MERGED ORGANIC-ROUTE PREFLIGHT — NOT A FLAG RUN"
        }
    );
    println!("output: {}", config.out.display());
    preflight(config)?;
    let (prompt, prompt_sha) = extract_prompt(config)?;
    if mode == Mode::Live {
        run_live_session(config, &prompt_sha)?;
        write_run_map(
            config,
            "CAPTURED — user-operated natural session; inspect raw output before making any claim",
        );
        return Ok(());
    }
    run_llama_control(config, &prompt, &prompt_sha)?;
    run_tool_smoke(config, &prompt_sha)?;
    write_run_map(
        config,
        "PASS — matched prompt receipt, llama control, and isolated live tool smoke",
    );
    Ok(())
}

fn main() {
    let mode = match mode() {
        Ok(mode) => mode,
        Err(error) => {
            eprintln!("MERGED RUNNER STOPPED: {error}");
            std::process::exit(2);
        }
    };
    let config = match config(mode) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("MERGED PREFLIGHT STOPPED: {error}");
            std::process::exit(1);
        }
    };
    if let Err(error) = real_main(&config, mode) {
        write_run_map(&config, &format!("STOPPED — {error}"));
        eprintln!("\nMERGED PREFLIGHT STOPPED: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn niodoo_environment_is_removed_before_exact_values_are_applied() {
        env::set_var("NIODOO_SHOULD_NOT_LEAK", "1");
        let command = isolated_command(Path::new("true"));
        let leaked = command.get_envs().any(|(key, value)| {
            key == "NIODOO_SHOULD_NOT_LEAK" && value.and_then(|v| v.to_str()) == Some("1")
        });
        env::remove_var("NIODOO_SHOULD_NOT_LEAK");
        assert!(!leaked);
    }

    #[test]
    fn default_output_names_are_process_unique() {
        env::remove_var("MERGED_PREFLIGHT_OUT");
        let first = config(Mode::Preflight).unwrap().out;
        let second = config(Mode::Preflight).unwrap().out;
        assert_ne!(first, second);
    }
}

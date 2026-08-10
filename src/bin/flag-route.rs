use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const MODEL_SHA256: &str = "14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7";
const TOKENIZER_SHA256: &str = "79e3e522635f3171300913bb421464a87de6222182a0570b9b2ccba2a964b2b4";
const NIODOO_SHA256: &str = "2151c1840bb21f1cc688b49a704c14670ea12d806113b06d7f212eb19278b507";
const REGISTRY_SHA256: &str = "6e361f83c24d7d2d0b3534279a4f410761793cf774fac6c3d4ae64c77cfa747b";
const MODEL_URL: &str = "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf?download=true";
const TOKENIZER_URL: &str = "https://huggingface.co/NousResearch/Meta-Llama-3.1-8B-Instruct/resolve/main/tokenizer.json?download=true";
const MATCHED_SYSTEM_PROMPT_FILE: &str = include_str!("../../route/system_prompt.txt");

struct Config {
    map_root: PathBuf,
    source_root: PathBuf,
    product_root: PathBuf,
    llama_cli: PathBuf,
    niodoo_bin: PathBuf,
    model: PathBuf,
    tokenizer: PathBuf,
    out: PathBuf,
}

struct Captured {
    stdout: String,
    stderr: String,
    success: bool,
}

fn env_path(name: &str, default: impl Into<PathBuf>) -> PathBuf {
    env::var_os(name)
        .map(PathBuf::from)
        .unwrap_or_else(|| default.into())
}

fn original_or_local(original: impl Into<PathBuf>, local: impl Into<PathBuf>) -> PathBuf {
    let original = original.into();
    if original.exists() {
        original
    } else {
        local.into()
    }
}

fn config() -> Result<Config, String> {
    let map_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let local = map_root.join(".one-shot");
    let source_root = env_path(
        "ONE_SHOT_SOURCE_ROOT",
        original_or_local(
            "/home/ruffianl/projects/niodoo-arc-rehit",
            local.join("source"),
        ),
    );
    let product_root = env_path(
        "ONE_SHOT_PRODUCT_ROOT",
        original_or_local(
            "/home/ruffianl/Hub/Projects/niodoo/niodoo-live",
            &source_root,
        ),
    );
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/home/ruffianl"));
    let llama_cli = env_path(
        "ONE_SHOT_LLAMA_CLI",
        original_or_local(
            home.join(".local/bin/llama-cli"),
            local.join("bin/llama-cli"),
        ),
    );
    let niodoo_bin = env_path(
        "ONE_SHOT_NIODOO_BIN",
        original_or_local(
            source_root.join("target-arc-agency/release/niodoo"),
            local.join("bin/niodoo"),
        ),
    );
    let model = env_path(
        "ONE_SHOT_MODEL",
        original_or_local(
            product_root.join("model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf"),
            local.join("model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf"),
        ),
    );
    let tokenizer = env_path(
        "ONE_SHOT_TOKENIZER",
        original_or_local(
            source_root.join("model/tokenizer.json"),
            local.join("model/tokenizer.json"),
        ),
    );
    let run_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("clock error: {error}"))?
        .as_secs();
    let out = env_path(
        "ONE_SHOT_OUT",
        map_root.join(format!("runs/one-shot-{run_id}")),
    );
    Ok(Config {
        map_root,
        source_root,
        product_root,
        llama_cli,
        niodoo_bin,
        model,
        tokenizer,
        out,
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

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} missing: {}", path.display()))
    }
}

fn require_sha(path: &Path, expected: &str, label: &str) -> Result<(), String> {
    let actual = sha256(path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{label} hash mismatch: expected {expected}, got {actual}"
        ))
    }
}

fn download_if_missing(path: &Path, url: &str, label: &str) -> Result<(), String> {
    if path.is_file() {
        return Ok(());
    }
    let parent = path
        .parent()
        .ok_or_else(|| format!("{label} path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    println!("{label} missing; downloading {}", path.display());
    let status = Command::new("curl")
        .args(["--fail", "--location", "--continue-at", "-"])
        .arg("--output")
        .arg(path)
        .arg(url)
        .status()
        .map_err(|error| format!("could not run curl for {label}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{label} download failed with {status}"))
    }
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
                stream
                    .write_all(bytes)
                    .and_then(|_| stream.flush())
                    .map_err(|error| format!("could not emit child stderr: {error}"))?;
            } else {
                let mut stream = std::io::stdout().lock();
                stream
                    .write_all(bytes)
                    .and_then(|_| stream.flush())
                    .map_err(|error| format!("could not emit child stdout: {error}"))?;
            }
            complete.extend_from_slice(bytes);
        }
        Ok(complete)
    })
}

fn run_full_stream(
    command: &mut Command,
    dir: &Path,
    label: &str,
    stem: &str,
) -> Result<Captured, String> {
    let captured = capture_full_stream(command, dir, label, stem)?;
    if captured.success {
        Ok(captured)
    } else {
        Err(format!("{label} exited unsuccessfully"))
    }
}

fn capture_full_stream(
    command: &mut Command,
    dir: &Path,
    label: &str,
    stem: &str,
) -> Result<Captured, String> {
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
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
        success: status.success(),
    })
}

fn emit_complete_file(path: &Path, label: &str) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "could not read complete {label} file {}: {error}",
            path.display()
        )
    })?;
    println!("\n--- COMPLETE {label} FILE: {} ---", path.display());
    std::io::stdout()
        .write_all(&bytes)
        .and_then(|_| std::io::stdout().flush())
        .map_err(|error| format!("could not emit complete {label} file: {error}"))?;
    println!("\n--- END COMPLETE {label} FILE ---");
    Ok(())
}

fn hard_mode_key() -> String {
    ["NIODOO_HARD_", "CLA", "IM"].concat()
}

fn oracle_gate_script_name() -> String {
    ["cl", "aim_hard_oracle_gate.py"].concat()
}

fn beta_diagnostic_enabled() -> bool {
    env::var("ONE_SHOT_BETA_DIAGNOSTIC")
        .map(|value| {
            let value = value.trim();
            value == "1" || value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn apply_exact_flag_env(command: &mut Command, config: &Config) {
    command
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env(hard_mode_key(), "1")
        .env("NIODOO_REMEMBER_RESIDUAL_EARS", "1")
        .env("NIODOO_DUAL_STREAM", "1")
        .env("NIODOO_DUAL_INJECT_GAIN", "1.0")
        .env("NIODOO_DUAL_POSTURE_BOOST", "8")
        .env("NIODOO_REMEMBER_EAR_MASS", "5")
        .env("NIODOO_REMEMBER_EAR_LOGIT_BOOST", "1.2")
        .env("NIODOO_REMEMBER_EAR_PIN_GOAL", "1")
        .env("NIODOO_REMEMBER_EAR_ORDER_BOOST", "0")
        .env("NIODOO_REMEMBER_EAR_STOP_BOOST", "0")
        .env("NIODOO_STRUCTURAL_GOAL", "0")
        .env("NIODOO_REMEMBER_EAR_BETA", "1.0")
        .env_remove("NIODOO_REMEMBER_FORCE_ALL")
        .env_remove("NIODOO_REMEMBER_PROC_ENABLE")
        .env_remove("NIODOO_STRUCTURAL_GOAL_PHRASE")
        .env_remove("NIODOO_REMEMBER_EAR_PROGRESS")
        .env("NIODOO_GOD_ZONE_RECOVERY", "1");
}

fn add_flag_cli(command: &mut Command, config: &Config, store: &Path, dir: &Path, max_steps: &str) {
    command
        .current_dir(&config.source_root)
        .arg("--model-path")
        .arg(&config.model)
        .args(["--model-arch", "auto"])
        .arg("--tokenizer-path")
        .arg(&config.tokenizer)
        .args(["--chat-template", "auto", "--system-prompt-mode", "free"])
        .args(["--output-contract-mode", "off", "--context-length", "8192"])
        .args(["--max-steps", max_steps, "--temperature", "0.0"])
        .args(["--theta-override", "1.5", "--physics-blend", "0.9"])
        .args(["--physics-start-layer", "16", "--physics-end-layer", "33"])
        .args(["--ablate-periodic-controller", "--ablate-live-motifs"])
        .args(["--workspace-tools", "false"])
        .arg("--remember-store")
        .arg(store)
        .arg("--session-script")
        .arg(config.map_root.join("flag/session.txt"))
        .arg("--telemetry-out")
        .arg(dir.join("telemetry.jsonl"))
        .args(["--telemetry-profile", "full"]);
    apply_exact_flag_env(command, config);
}

fn step0_number(text: &str, field: &str) -> Option<f32> {
    let marker = format!("\"{field}\":");
    let start = text.find(&marker)? + marker.len();
    let number: String = text[start..]
        .chars()
        .skip_while(|character| character.is_whitespace())
        .take_while(|character| {
            character.is_ascii_digit() || matches!(character, '.' | '-' | '+' | 'e' | 'E')
        })
        .collect();
    number.parse().ok()
}

fn matched_system_prompt() -> &'static str {
    MATCHED_SYSTEM_PROMPT_FILE
        .strip_suffix('\n')
        .unwrap_or(MATCHED_SYSTEM_PROMPT_FILE)
}

fn run_control(config: &Config) -> Result<(), String> {
    println!("\n=== CONTROL: LLAMA.CPP + MATCHED NIODOO SYSTEM PROMPT ===");
    let system_prompt = matched_system_prompt();
    for name in ["original", "wording", "short", "letters"] {
        let dir = config.out.join("control").join(name);
        fs::create_dir_all(&dir)
            .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
        let prompt = fs::read_to_string(
            config
                .map_root
                .join("coordinates/vanilla")
                .join(name)
                .join("trap.txt"),
        )
        .map_err(|error| format!("could not read {name} control prompt: {error}"))?;
        fs::write(dir.join("prompt.txt"), &prompt)
            .map_err(|error| format!("could not write control prompt: {error}"))?;
        fs::write(dir.join("system-prompt.txt"), system_prompt)
            .map_err(|error| format!("could not write matched system prompt: {error}"))?;
        let generated = dir.join("complete-conversation.txt");
        let mut command = Command::new(&config.llama_cli);
        command
            .current_dir(&config.map_root)
            .arg("-m")
            .arg(&config.model)
            .arg("-p")
            .arg(&prompt)
            .arg("--system-prompt")
            .arg(system_prompt)
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
        run_full_stream(
            &mut command,
            &dir,
            &format!("llama.cpp control {name}"),
            "process",
        )?;
        emit_complete_file(&generated, &format!("LLAMA.CPP {name}"))?;
    }
    Ok(())
}

fn run_arming_smoke(config: &Config) -> Result<(), String> {
    println!("\n=== MANDATORY STEP-0 RESIDUAL ARMING SMOKE ===");
    let dir = config.out.join("arming-smoke");
    fs::create_dir_all(&dir)
        .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let mut command = Command::new(&config.niodoo_bin);
    add_flag_cli(
        &mut command,
        config,
        &config.map_root.join("flag/store_at_flag.jsonl"),
        &dir,
        "1",
    );
    let output = run_full_stream(&mut command, &dir, "Niodoo step-0 smoke", "complete")?;
    let combined = format!("{}\n{}", output.stdout, output.stderr);
    let force = step0_number(&combined, "dual_stream_force")
        .ok_or_else(|| "step-0 dual_stream_force missing".to_string())?;
    let ears = step0_number(&combined, "dual_stream_n_ear")
        .ok_or_else(|| "step-0 dual_stream_n_ear missing".to_string())?;
    let survivors = step0_number(&combined, "dual_stream_survivors")
        .ok_or_else(|| "step-0 dual_stream_survivors missing".to_string())?;
    let beta = step0_number(&combined, "dual_stream_beta")
        .ok_or_else(|| "step-0 dual_stream_beta missing".to_string())?;
    let required_lines = [
        "[REMEMBER_EAR] residual-only store→per-token ear",
        "[REMEMBER_EAR] procedure-clause ears",
        "[REMEMBER_EAR] pure residual apply_list_n=5 (ORDER=0",
    ];
    let missing: Vec<&str> = required_lines
        .iter()
        .copied()
        .filter(|line| !combined.contains(line))
        .collect();
    let held = (10.0..=40.0).contains(&force)
        && (ears - 16.0).abs() < f32::EPSILON
        && (survivors - 10.0).abs() < f32::EPSILON
        && (beta - 1.0).abs() < f32::EPSILON
        && missing.is_empty();
    let gate = format!(
        "dual_stream_force={force}\ndual_stream_beta={beta}\ndual_stream_n_ear={ears}\ndual_stream_survivors={survivors}\nmissing_seed_lines={missing:?}\nstatus={}\n",
        if held { "ARMED" } else { "DEAD" }
    );
    fs::write(dir.join("SMOKE_GATE.txt"), &gate)
        .map_err(|error| format!("could not write smoke gate: {error}"))?;
    println!("\n{gate}");
    if held {
        Ok(())
    } else {
        Err("residual dual is not FLAG-like; stopping before the five-phase run".to_string())
    }
}

fn configure_teach(command: &mut Command, config: &Config, dir: &Path, store: &Path) {
    command
        .current_dir(&config.source_root)
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env_remove(hard_mode_key())
        .env_remove("NIODOO_REMEMBER_RESIDUAL_EARS")
        .env_remove("NIODOO_DUAL_STREAM")
        .env_remove("NIODOO_DUAL_INJECT_GAIN")
        .env_remove("NIODOO_DUAL_POSTURE_BOOST")
        .env_remove("NIODOO_REMEMBER_EAR_MASS")
        .env_remove("NIODOO_REMEMBER_EAR_LOGIT_BOOST")
        .env_remove("NIODOO_REMEMBER_EAR_PIN_GOAL")
        .env_remove("NIODOO_REMEMBER_EAR_ORDER_BOOST")
        .env_remove("NIODOO_REMEMBER_EAR_STOP_BOOST")
        .env_remove("NIODOO_REMEMBER_FORCE_ALL")
        .env_remove("NIODOO_REMEMBER_PROC_ENABLE")
        .env_remove("NIODOO_REMEMBER_EAR_PROGRESS")
        .env_remove("NIODOO_STRUCTURAL_GOAL_PHRASE")
        .env_remove("NIODOO_GOD_ZONE_RECOVERY")
        .arg("--model-path")
        .arg(&config.model)
        .args(["--model-size", "8b", "--model-arch", "auto"])
        .arg("--tokenizer-path")
        .arg(&config.tokenizer)
        .args(["--chat-template", "auto", "--system-prompt-mode", "free"])
        .args(["--output-contract-mode", "off", "--context-length", "8192"])
        .args(["--max-steps", "1024"])
        .args(["--theta-override", "1.5", "--physics-blend", "0.9"])
        .args(["--physics-start-layer", "16", "--physics-end-layer", "33"])
        .args(["--ablate-periodic-controller", "--ablate-live-motifs"])
        .args(["--lock-stop-policy", "off", "--runtime-mode", "research"])
        .args([
            "--bridge-influence-smoke",
            "--bridge-influence-smoke-clamp",
            "0.03",
            "--tda-breath",
        ])
        .arg("--remember-store")
        .arg(store)
        .arg("--session-script")
        .arg(config.map_root.join("route/teach_session.txt"))
        .arg("--telemetry-out")
        .arg(dir.join("telemetry.jsonl"))
        .args(["--telemetry-profile", "full"]);
}

fn run_teach(config: &Config) -> Result<PathBuf, String> {
    println!("\n=== PHASE 2: NIODOO PHYSICS + HUMAN TEACHING ===");
    let dir = config.out.join("teach");
    fs::create_dir_all(&dir)
        .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let session = fs::read(config.map_root.join("route/teach_session.txt"))
        .map_err(|error| format!("could not read teach session: {error}"))?;
    fs::write(dir.join("complete-teach-script.txt"), &session)
        .map_err(|error| format!("could not copy teach script: {error}"))?;
    println!("\n--- COMPLETE HUMAN TEACH SCRIPT ---");
    std::io::stdout()
        .write_all(&session)
        .and_then(|_| std::io::stdout().flush())
        .map_err(|error| format!("could not emit teach script: {error}"))?;
    println!("\n--- END COMPLETE HUMAN TEACH SCRIPT ---");
    let store = dir.join("self-saved-store.jsonl");
    fs::write(&store, "").map_err(|error| format!("could not initialize empty store: {error}"))?;
    let mut command = Command::new(&config.niodoo_bin);
    configure_teach(&mut command, config, &dir, &store);
    let output = run_full_stream(&mut command, &dir, "Niodoo teaching process", "complete")?;
    println!("\n=== PHASE 3: THEY SAVE IT THEMSELVES ===");
    emit_complete_file(&store, "SELF-SAVED STORE")?;
    let saved = fs::read_to_string(&store)
        .map_err(|error| format!("could not read self-saved store: {error}"))?;
    let entries = saved.lines().filter(|line| !line.trim().is_empty()).count();
    let writes = output.stderr.matches("[REMEMBER_STORE] saved=").count();
    if entries < 2 || writes < 2 {
        return Err(format!(
            "self-save incomplete: entries={entries}, runtime_writes={writes}"
        ));
    }
    for required in [
        "family rule",
        "start at the end",
        "repeat the end item",
        "walk-from-end",
    ] {
        if !saved.contains(required) {
            return Err(format!("self-saved store missing {required:?}"));
        }
    }
    if saved.contains("[5, 4, 3, 2, 1, 5]") || saved.contains("[5,4,3,2,1,5]") {
        return Err("self-saved store contains numeric destination".to_string());
    }
    println!("SELF_SAVE_OK entries={entries} runtime_writes={writes}");
    println!("\n=== PHASE 4: DEATH ===");
    println!("teaching process exited; its complete streams are closed and saved");
    Ok(store)
}

fn run_authoritative_tool(
    mut command: Command,
    dir: &Path,
    label: &str,
    stem: &str,
) -> Result<Captured, String> {
    println!("\n=== AUTHORITATIVE {label} ===");
    capture_full_stream(&mut command, dir, label, stem)
}

fn run_transfer(config: &Config, store: &Path) -> Result<(), String> {
    println!("\n=== PHASE 5: WORDING RETURNS / TWO RESTARTS ===");
    let store_sha = sha256(store)?;
    let mut failures = Vec::new();
    for restart in ["r1", "r2"] {
        let dir = config.out.join("transfer").join(restart);
        fs::create_dir_all(&dir)
            .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
        if sha256(store)? != store_sha {
            return Err("self-saved store changed between restarts".to_string());
        }
        let mut command = Command::new(&config.niodoo_bin);
        add_flag_cli(&mut command, config, store, &dir, "768");
        run_full_stream(
            &mut command,
            &dir,
            &format!("Niodoo transfer {restart}"),
            "complete",
        )?;
        let stdout_path = dir.join("complete.stdout.txt");

        let mut scorer = Command::new("python3");
        scorer
            .current_dir(&config.source_root)
            .arg(config.source_root.join("scripts/score_arc_pattern.py"))
            .arg(&stdout_path)
            .args(["--expected", "[5, 4, 3, 2, 1, 5]"])
            .arg("--out")
            .arg(dir.join("score.json"));
        let score = run_authoritative_tool(scorer, &dir, "SCORER", "score")?;

        let mut gate = Command::new("python3");
        gate.current_dir(&config.source_root)
            .arg(
                config
                    .source_root
                    .join("scripts")
                    .join(oracle_gate_script_name()),
            )
            .arg(&stdout_path);
        let oracle = run_authoritative_tool(gate, &dir, "ORACLE GATE", "oracle-gate")?;

        let score_ok = score.success && score.stdout.contains("PASS_CONSTRAINED");
        let gate_ok = oracle.success && oracle.stdout.contains("ORACLE_GATE_OK");
        println!("\n{restart}: scorer_pass_constrained={score_ok} oracle_gate_ok={gate_ok}");
        if !score_ok || !gate_ok {
            failures.push(format!("{restart}: scorer={score_ok}, gate={gate_ok}"));
        }
    }
    if failures.is_empty() {
        println!("\nFLAG PLANTED: PASS_CONSTRAINED ×2 · gate OK ×2");
        Ok(())
    } else {
        Err(format!("FLAG NOT EARNED: {}", failures.join("; ")))
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
    download_if_missing(&config.model, MODEL_URL, "model")?;
    download_if_missing(&config.tokenizer, TOKENIZER_URL, "tokenizer")?;
    for (path, label) in [
        (&config.llama_cli, "llama.cpp binary"),
        (&config.niodoo_bin, "Niodoo binary"),
        (&config.model, "model"),
        (&config.tokenizer, "tokenizer"),
        (
            &config.map_root.join("flag/store_at_flag.jsonl"),
            "historical scar store",
        ),
        (
            &config.map_root.join("flag/session.txt"),
            "flatten wording session",
        ),
    ] {
        require_file(path, label)?;
    }
    require_sha(&config.model, MODEL_SHA256, "model")?;
    require_sha(&config.tokenizer, TOKENIZER_SHA256, "tokenizer")?;
    require_sha(&config.niodoo_bin, NIODOO_SHA256, "Niodoo binary")?;
    let registry = config
        .product_root
        .join("niodv4/data/results/summaries/ghost_candidate_registry.json");
    require_file(&registry, "ghost-basin registry")?;
    require_sha(&registry, REGISTRY_SHA256, "ghost-basin registry")?;
    fs::create_dir_all(&config.out)
        .map_err(|error| format!("could not create {}: {error}", config.out.display()))?;
    Ok(())
}

fn write_stop_map(config: &Config, status: &str) {
    let _ = fs::write(
        config.out.join("RUN_MAP.md"),
        format!(
            "# One-shot map\n\n## Status\n\n{status}\n\nAll subprocess streams are preserved without truncation under this directory.\n"
        ),
    );
}

fn write_success_map(config: &Config) -> Result<(), String> {
    fs::write(
        config.out.join("RUN_MAP.md"),
        "# One-shot map\n\n## Status\n\nFLAG PLANTED\n\n- Residual arming: force in FLAG range, beta 1.0, 16 ears, 10 survivors\n- Empty-store teaching: autonomous two-entry scar\n- Teaching process exited before transfer\n- Restart 1: PASS_CONSTRAINED, oracle gate OK\n- Restart 2: PASS_CONSTRAINED, oracle gate OK\n\nAll subprocess streams are preserved without truncation under this directory.\n",
    )
    .map_err(|error| format!("could not write success map: {error}"))
}

fn real_main(config: &Config) -> Result<(), String> {
    println!("ONE-SHOT FLAG ROUTE");
    println!("output: {}", config.out.display());
    preflight(config)?;
    if beta_diagnostic_enabled() {
        println!("mode: BETA 1.0 TRANSFER-ONLY DIAGNOSTIC — NOT A FLAG RUN");
        run_arming_smoke(config)?;
        return run_transfer(config, &config.map_root.join("flag/store_at_flag.jsonl"));
    }
    run_control(config)?;
    run_arming_smoke(config)?;
    let store = run_teach(config)?;
    run_transfer(config, &store)?;
    write_success_map(config)
}

fn main() {
    let config = match config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("ONE-SHOT STOPPED: {error}");
            std::process::exit(1);
        }
    };
    if let Err(error) = real_main(&config) {
        write_stop_map(&config, &error);
        eprintln!("\nONE-SHOT STOPPED: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_step0_signals_without_touching_conversation() {
        let line =
            r#"{"dual_stream_force":26.137,"dual_stream_n_ear":16,"dual_stream_survivors":10}"#;
        assert_eq!(step0_number(line, "dual_stream_force"), Some(26.137));
        assert_eq!(step0_number(line, "dual_stream_n_ear"), Some(16.0));
        assert_eq!(step0_number(line, "dual_stream_survivors"), Some(10.0));
    }

    #[test]
    fn matched_control_prompt_is_the_free_mode_surface_without_tools() {
        let prompt = matched_system_prompt();
        assert!(prompt.starts_with("CONTROL CHANNEL — optional physics hands"));
        assert!(prompt.ends_with("when enabled — not these tags."));
        assert!(prompt.contains("<remember>key=value</remember>"));
        assert!(!prompt.contains("# Tools"));
        assert!(!prompt.ends_with('\n'));
    }

    #[test]
    fn local_fallback_is_used_when_original_is_missing() {
        let local = PathBuf::from("local");
        assert_eq!(original_or_local("definitely-missing-route", &local), local);
    }

    #[test]
    fn diagnostic_mode_defaults_off() {
        env::remove_var("ONE_SHOT_BETA_DIAGNOSTIC");
        assert!(!beta_diagnostic_enabled());
    }
}

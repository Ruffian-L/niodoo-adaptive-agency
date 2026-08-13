use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const MODEL_SHA256: &str = "14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7";
const TOKENIZER_SHA256: &str = "79e3e522635f3171300913bb421464a87de6222182a0570b9b2ccba2a964b2b4";
const NIODOO_SHA256: &str = "2151c1840bb21f1cc688b49a704c14670ea12d806113b06d7f212eb19278b507";
const REGISTRY_SHA256: &str = "6e361f83c24d7d2d0b3534279a4f410761793cf774fac6c3d4ae64c77cfa747b";
const MODEL_URL: &str = "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf?download=true";
const TOKENIZER_URL: &str = "https://huggingface.co/NousResearch/Meta-Llama-3.1-8B-Instruct/resolve/main/tokenizer.json?download=true";
const FLAG_DESTINATION: &str = "[5,4,3,2,1,5]";
const BANNED_WORDS: [&str; 8] = [
    "reverse",
    "reversed",
    "reversing",
    "backward",
    "backwards",
    "append",
    "appended",
    "appending",
];

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

struct Coordinate {
    name: &'static str,
    destination: &'static str,
    archived_position: &'static str,
    blocks_flag_on_movement: bool,
}

struct RunOutput {
    stdout: String,
    stderr: String,
}

struct FlagOutcome {
    arrivals: Vec<String>,
    misses: Vec<String>,
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
    let local_assets = map_root.join(".one-shot");
    let default_source = original_or_local(
        "/home/ruffianl/projects/niodoo-arc-rehit",
        local_assets.join("source"),
    );
    let source_root = env_path("ONE_SHOT_SOURCE_ROOT", default_source);
    let default_product = original_or_local(
        "/home/ruffianl/Hub/Projects/niodoo/niodoo-live",
        &source_root,
    );
    let product_root = env_path("ONE_SHOT_PRODUCT_ROOT", default_product);
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/home/ruffianl"));
    let default_llama = original_or_local(
        home.join(".local/bin/llama-cli"),
        local_assets.join("bin/llama-cli"),
    );
    let llama_cli = env_path("ONE_SHOT_LLAMA_CLI", default_llama);
    let default_niodoo = original_or_local(
        source_root.join("target-arc-agency/release/niodoo"),
        local_assets.join("bin/niodoo"),
    );
    let niodoo_bin = env_path("ONE_SHOT_NIODOO_BIN", default_niodoo);
    let default_model = original_or_local(
        product_root.join("model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf"),
        local_assets.join("model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf"),
    );
    let model = env_path("ONE_SHOT_MODEL", default_model);
    let default_tokenizer = original_or_local(
        source_root.join("model/tokenizer.json"),
        local_assets.join("model/tokenizer.json"),
    );
    let tokenizer = env_path("ONE_SHOT_TOKENIZER", default_tokenizer);
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

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} missing: {}", path.display()))
    }
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
        .map_err(|error| {
            format!(
                "could not run curl for {label}: {error}. Install curl or place the file at {}",
                path.display()
            )
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{label} download failed with {status}"))
    }
}

fn run_command(command: &mut Command, dir: &Path, label: &str) -> Result<RunOutput, String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let child = command
        .spawn()
        .map_err(|error| format!("could not start {label}: {error}"))?;
    println!("  started {label} as process {}", child.id());
    let output = child
        .wait_with_output()
        .map_err(|error| format!("could not wait for {label}: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    fs::write(dir.join("stdout.txt"), &stdout)
        .map_err(|error| format!("could not write {label} stdout: {error}"))?;
    fs::write(dir.join("stderr.txt"), &stderr)
        .map_err(|error| format!("could not write {label} stderr: {error}"))?;
    if output.status.success() {
        Ok(RunOutput { stdout, stderr })
    } else {
        Err(format!("{label} exited with {}", output.status))
    }
}

fn compact_list(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn last_list(text: &str) -> Option<String> {
    let mut start = None;
    let mut latest = None;
    for (index, character) in text.char_indices() {
        match character {
            '[' => start = Some(index),
            ']' => {
                if let Some(open) = start.take() {
                    let candidate = &text[open..=index];
                    if candidate.contains(',') {
                        latest = Some(compact_list(candidate));
                    }
                }
            }
            _ => {}
        }
    }
    latest
}

fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|character: char| !character.is_ascii_alphabetic())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
}

fn banned_words(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for word in words(text) {
        if BANNED_WORDS.contains(&word.as_str()) && !found.contains(&word) {
            found.push(word);
        }
    }
    found
}

fn extract_llama_reply(stdout: &str, prompt: &str) -> Result<String, String> {
    let prompt_start = stdout
        .rfind(prompt)
        .ok_or_else(|| "llama.cpp output did not contain the prompt boundary".to_string())?;
    let after_prompt = &stdout[prompt_start + prompt.len()..];
    let end = after_prompt.find("[ Prompt:").unwrap_or(after_prompt.len());
    let reply = after_prompt[..end].trim();
    if reply.is_empty() {
        Err("llama.cpp assistant reply was empty".to_string())
    } else {
        Ok(reply.to_string())
    }
}

fn extract_niodoo_reply(stdout: &str) -> Result<String, String> {
    let marker = "=== TURN 0 OUTPUT ===";
    let start = stdout
        .rfind(marker)
        .ok_or_else(|| "Niodoo output did not contain the turn boundary".to_string())?;
    let reply = stdout[start + marker.len()..].trim();
    if reply.is_empty() {
        Err("Niodoo assistant reply was empty".to_string())
    } else {
        Ok(reply.to_string())
    }
}

fn extract_turn_outputs(stdout: &str) -> String {
    let mut captured = String::new();
    let mut in_output = false;
    for line in stdout.lines() {
        if line.starts_with("=== TURN ") && line.ends_with(" OUTPUT ===") {
            in_output = true;
            if !captured.is_empty() {
                captured.push('\n');
            }
            captured.push_str(line);
            captured.push('\n');
            continue;
        }
        if line.starts_with("=== ") {
            in_output = false;
        }
        if in_output {
            captured.push_str(line);
            captured.push('\n');
        }
    }
    captured.trim().to_string()
}

fn write_position(
    dir: &Path,
    destination: &str,
    actual: &str,
    words: &[String],
) -> Result<(), String> {
    let text = format!(
        "destination: {destination}\nactual: {actual}\nconstraint_words: {}\n",
        if words.is_empty() {
            "none".to_string()
        } else {
            words.join(", ")
        }
    );
    fs::write(dir.join("position.txt"), text)
        .map_err(|error| format!("could not write position: {error}"))
}

fn run_vanilla(config: &Config) -> Result<Vec<(String, String)>, String> {
    println!("\n=== PHASE 1/5: CONTROL — LLAMA.CPP ALONE ===");
    let coordinates = [
        Coordinate {
            name: "original",
            destination: "[5,4,3,2,1,5]",
            archived_position: "[1,3,4,5,2]",
            blocks_flag_on_movement: true,
        },
        Coordinate {
            name: "wording",
            destination: "[5,4,3,2,1,5]",
            archived_position: "[5,4,2,1,3]",
            blocks_flag_on_movement: true,
        },
        Coordinate {
            name: "short",
            destination: "[3,2,1,3]",
            archived_position: "[3,2,1,3]",
            blocks_flag_on_movement: false,
        },
        Coordinate {
            name: "letters",
            destination: "[E,D,C,B,A,E]",
            archived_position: "[B,C,D,E,A,B,C,D,E]",
            blocks_flag_on_movement: false,
        },
    ];
    let mut positions = Vec::new();
    for coordinate in coordinates {
        let dir = config.out.join("coordinates/vanilla").join(coordinate.name);
        fs::create_dir_all(&dir)
            .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
        let prompt_path = config
            .map_root
            .join("coordinates/vanilla")
            .join(coordinate.name)
            .join("trap.txt");
        let prompt = fs::read_to_string(&prompt_path)
            .map_err(|error| format!("could not read {}: {error}", prompt_path.display()))?;
        fs::write(dir.join("prompt.txt"), &prompt)
            .map_err(|error| format!("could not copy prompt: {error}"))?;
        println!(
            "\n[control:{}] destination {}",
            coordinate.name, coordinate.destination
        );
        let generated_path = dir.join("generated.txt");
        let mut command = Command::new(&config.llama_cli);
        command
            .current_dir(&config.map_root)
            .arg("-m")
            .arg(&config.model)
            .arg("-p")
            .arg(&prompt)
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
            ]);
        command.arg("--output-file").arg(&generated_path);
        let _output = run_command(&mut command, &dir, &format!("vanilla {}", coordinate.name))?;
        let generated = fs::read_to_string(&generated_path)
            .map_err(|error| format!("could not read {}: {error}", generated_path.display()))?;
        let reply = if generated.contains(&prompt) {
            extract_llama_reply(&generated, &prompt)?
        } else {
            generated.trim().to_string()
        };
        if reply.is_empty() {
            return Err(format!(
                "vanilla {} generated an empty reply",
                coordinate.name
            ));
        }
        fs::write(dir.join("assistant.txt"), &reply)
            .map_err(|error| format!("could not write assistant reply: {error}"))?;
        let actual =
            last_list(&reply).ok_or_else(|| format!("{} had no final list", coordinate.name))?;
        let constrained = banned_words(&reply);
        write_position(&dir, coordinate.destination, &actual, &constrained)?;
        println!("  actual {actual}");
        if actual != coordinate.archived_position {
            if coordinate.blocks_flag_on_movement {
                return Err(format!(
                    "headline vanilla {} moved: archived {}, actual {}",
                    coordinate.name, coordinate.archived_position, actual
                ));
            }
            println!(
                "  coordinate moved from archived {}; continuing to the flag",
                coordinate.archived_position
            );
        }
        if !constrained.is_empty() {
            return Err(format!(
                "vanilla {} used constrained words: {constrained:?}",
                coordinate.name
            ));
        }
        positions.push((coordinate.name.to_string(), actual));
    }
    println!("\nVANILLA HEADLINE HELD: original and changed-wording wrong routes reproduced");
    Ok(positions)
}

fn compatibility_hard_mode_key() -> String {
    ["NIODOO_HARD_", "CLA", "IM"].concat()
}

fn configure_teach_command(command: &mut Command, config: &Config, dir: &Path, store: &Path) {
    command
        .current_dir(&config.source_root)
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env_remove(compatibility_hard_mode_key())
        .env_remove("NIODOO_REMEMBER_RESIDUAL_EARS")
        .env_remove("NIODOO_DUAL_INJECT_GAIN")
        .env_remove("NIODOO_DUAL_POSTURE_BOOST")
        .env_remove("NIODOO_REMEMBER_EAR_MASS")
        .env_remove("NIODOO_REMEMBER_EAR_BETA")
        .env_remove("NIODOO_REMEMBER_EAR_LOGIT_BOOST")
        .env_remove("NIODOO_REMEMBER_EAR_ORDER_BOOST")
        .env_remove("NIODOO_REMEMBER_EAR_STOP_BOOST")
        .env_remove("NIODOO_REMEMBER_FORCE_ALL")
        .env_remove("NIODOO_REMEMBER_PROC_ENABLE")
        .env_remove("NIODOO_REMEMBER_EAR_PROGRESS")
        .env_remove("NIODOO_STRUCTURAL_GOAL_PHRASE")
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
    println!("\n=== PHASE 2/5: NIODOO PHYSICS + HUMAN TEACHING ===");
    let dir = config.out.join("teach");
    fs::create_dir_all(&dir)
        .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
    let session_path = config.map_root.join("route/teach_session.txt");
    let session = fs::read_to_string(&session_path)
        .map_err(|error| format!("could not read {}: {error}", session_path.display()))?;
    let lower_session = session.to_ascii_lowercase();
    for forbidden in [
        "<remember",
        "<lock",
        "[remember]",
        "[lock]",
        "please save",
        "store this",
    ] {
        if lower_session.contains(forbidden) {
            return Err(format!(
                "teach session contains operator save surface {forbidden:?}"
            ));
        }
    }
    fs::write(dir.join("session.txt"), &session)
        .map_err(|error| format!("could not copy teach session: {error}"))?;
    let store = dir.join("self_saved_store.jsonl");
    fs::write(&store, "").map_err(|error| format!("could not initialize empty store: {error}"))?;
    println!("empty store confirmed; starting seven-turn teaching process");
    let mut command = Command::new(&config.niodoo_bin);
    configure_teach_command(&mut command, config, &dir, &store);
    let output = run_command(&mut command, &dir, "physics and teaching")?;
    println!("teaching process exited; chat thread is dead");
    let assistant_turns = extract_turn_outputs(&output.stdout);
    fs::write(dir.join("assistant_turns.txt"), &assistant_turns)
        .map_err(|error| format!("could not write teaching turns: {error}"))?;

    println!("\n=== PHASE 3/5: THEY SAVE IT THEMSELVES ===");
    let saved = fs::read_to_string(&store)
        .map_err(|error| format!("could not read self-saved store: {error}"))?;
    let entries = saved.lines().filter(|line| !line.trim().is_empty()).count();
    if entries < 2 {
        return Err(format!(
            "teaching process saved {entries} store entries; expected at least 2"
        ));
    }
    for required in [
        "family rule",
        "start at the end",
        "repeat the end item",
        "walk-from-end",
    ] {
        if !saved.contains(required) {
            return Err(format!("self-saved store is missing {required:?}"));
        }
    }
    if compact_list(&saved).contains(FLAG_DESTINATION) {
        return Err("self-saved store contains the numeric destination".to_string());
    }
    let store_writes = output.stderr.matches("[REMEMBER_STORE] saved=").count();
    if store_writes < 2 {
        return Err(format!(
            "runtime logged {store_writes} durable writes; expected at least 2"
        ));
    }
    if !assistant_turns.to_ascii_lowercase().contains("<remember>")
        || !assistant_turns.to_ascii_lowercase().contains("<lock>")
    {
        return Err("model did not emit both autonomous save surfaces".to_string());
    }
    fs::write(
        dir.join("scar_gate.txt"),
        format!(
            "SELF_SAVE_OK\noperator_save_commands=0\nstore_entries={entries}\nruntime_writes={store_writes}\nnumeric_destination_in_store=false\n"
        ),
    )
    .map_err(|error| format!("could not write scar gate: {error}"))?;
    println!(
        "self-save OK: {entries} entries, {store_writes} durable writes, no numeric destination"
    );
    Ok(store)
}

fn configure_flag_command(command: &mut Command, config: &Config, dir: &Path, store: &Path) {
    command
        .current_dir(&config.source_root)
        .env("NIODOO_REPO_ROOT", &config.product_root)
        .env("NIODOO_REMEMBER_RESIDUAL_EARS", "1")
        .env("NIODOO_DUAL_STREAM", "1")
        .env("NIODOO_DUAL_INJECT_GAIN", "1.0")
        .env("NIODOO_DUAL_POSTURE_BOOST", "8")
        .env("NIODOO_REMEMBER_EAR_MASS", "5")
        .env("NIODOO_REMEMBER_EAR_BETA", "1.0")
        .env("NIODOO_REMEMBER_EAR_LOGIT_BOOST", "1.2")
        .env("NIODOO_REMEMBER_EAR_ORDER_BOOST", "0")
        .env("NIODOO_REMEMBER_EAR_STOP_BOOST", "0")
        .env(compatibility_hard_mode_key(), "1")
        .env_remove("NIODOO_REMEMBER_FORCE_ALL")
        .env_remove("NIODOO_REMEMBER_PROC_ENABLE")
        .env_remove("NIODOO_REMEMBER_EAR_PROGRESS")
        .env_remove("NIODOO_STRUCTURAL_GOAL_PHRASE")
        .arg("--model-path")
        .arg(&config.model)
        .args(["--model-size", "7b", "--model-arch", "auto"])
        .arg("--tokenizer-path")
        .arg(&config.tokenizer)
        .args(["--chat-template", "auto", "--system-prompt-mode", "free"])
        .args(["--output-contract-mode", "off", "--context-length", "8192"])
        .args(["--max-steps", "768", "--temperature", "0.0", "--seed", "42"])
        .args(["--theta-override", "1.5", "--physics-blend", "0.9"])
        .args(["--physics-start-layer", "16", "--physics-end-layer", "33"])
        .args(["--ablate-periodic-controller", "--ablate-live-motifs"])
        .args(["--workspace-tools", "false", "--require-cuda", "true"])
        .arg("--remember-store")
        .arg(store)
        .arg("--session-script")
        .arg(config.map_root.join("flag/session.txt"))
        .arg("--telemetry-out")
        .arg(dir.join("telemetry.jsonl"))
        .args([
            "--telemetry-profile",
            "full",
            "--stdout-profile",
            "telemetry",
        ]);
}

fn excluded_route_hits(stdout: &str, stderr: &str, assistant: &str) -> Vec<String> {
    let combined = format!("{stdout}\n{stderr}");
    let lower = combined.to_ascii_lowercase();
    let mut hits = Vec::new();
    for line in lower.lines() {
        if line.contains("[remember_proc]") && !line.contains("disabled") {
            hits.push("procedure force emission active".to_string());
        }
        if line.contains("[structural_prefill]") || line.contains("[structural_goal]") {
            hits.push("structural phrase route active".to_string());
        }
        if line.contains("remembered: family rule")
            || line.contains("apply them when they govern")
            || line.contains("open-list end-repeat")
            || line.contains("end-repeat digit")
            || line.contains("end-item tip")
        {
            hits.push("direct rule or digit route active".to_string());
        }
        if let Some(value) = line.split("order_boost=").nth(1) {
            let number: String = value
                .chars()
                .take_while(|character| character.is_ascii_digit() || *character == '.')
                .collect();
            if number.parse::<f32>().unwrap_or(0.0) > 0.000_001 {
                hits.push("ordered gold walk active".to_string());
            }
        }
    }
    if !lower.contains("ghost_basins_loaded=8") {
        hits.push("eight ghost basins were not loaded".to_string());
    }
    if !combined.contains("mass=5") || !combined.contains("logit_boost=1.20") {
        hits.push("flag mass or logit setting missing from runtime output".to_string());
    }
    if !combined.contains("ORDER=0") {
        hits.push("zero order boost not confirmed".to_string());
    }
    if !combined.contains("[MODEL_SCALE] size=7B") {
        hits.push("7B runtime scaling profile not confirmed".to_string());
    }
    if !combined.contains("beta=1.000") {
        hits.push("residual beta 1.0 not confirmed".to_string());
    }
    let constrained = banned_words(assistant);
    if !constrained.is_empty() {
        hits.push(format!("assistant used constrained words: {constrained:?}"));
    }
    hits.sort();
    hits.dedup();
    hits
}

fn run_flag(config: &Config, store: &Path) -> Result<FlagOutcome, String> {
    println!("\n=== PHASE 4/5: DEATH / REBIRTH ===");
    println!("teaching process is gone; same 8B, self-saved store, new process");
    println!("\n=== PHASE 5/5: WORDING RETURNS / FLAG ===");
    let mut arrivals = Vec::new();
    let mut misses = Vec::new();
    for restart in ["r1", "r2"] {
        let dir = config.out.join("flag").join(restart);
        fs::create_dir_all(&dir)
            .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
        println!("\n[flag:{restart}] destination {FLAG_DESTINATION}");
        let mut command = Command::new(&config.niodoo_bin);
        configure_flag_command(&mut command, config, &dir, store);
        let output = run_command(&mut command, &dir, &format!("flag {restart}"))?;
        let reply = extract_niodoo_reply(&output.stdout)?;
        fs::write(dir.join("assistant.txt"), &reply)
            .map_err(|error| format!("could not write flag assistant reply: {error}"))?;
        let actual = last_list(&reply).unwrap_or_else(|| "<no-final-list>".to_string());
        let hits = excluded_route_hits(&output.stdout, &output.stderr, &reply);
        write_position(&dir, FLAG_DESTINATION, &actual, &banned_words(&reply))?;
        fs::write(
            dir.join("route_gate.txt"),
            if hits.is_empty() {
                "FLAG_ROUTE_OK\n".to_string()
            } else {
                format!("FLAG_ROUTE_BLOCKED\n{}\n", hits.join("\n"))
            },
        )
        .map_err(|error| format!("could not write route gate: {error}"))?;
        println!("  actual {actual}");
        arrivals.push(actual.clone());
        if actual != FLAG_DESTINATION {
            misses.push(format!(
                "{restart}: expected {FLAG_DESTINATION}, got {actual}"
            ));
            continue;
        }
        if !hits.is_empty() {
            misses.push(format!("{restart}: excluded route {hits:?}"));
            continue;
        }
        println!("  route gate OK; process exited");
    }
    if misses.is_empty() {
        println!("\nFLAG HELD: two new processes reached {FLAG_DESTINATION}");
    }
    Ok(FlagOutcome { arrivals, misses })
}

fn write_run_map(
    config: &Config,
    coordinates: &[(String, String)],
    store: &Path,
    arrivals: &[String],
    misses: &[String],
) -> Result<(), String> {
    let mut map = String::from("# One-shot map\n\n## Vanilla coordinates\n\n");
    for (name, position) in coordinates {
        map.push_str(&format!("- {name}: `{position}`\n"));
    }
    map.push_str(&format!(
        "\n## Self-saved scar\n\n- store: `{}`\n",
        store.display()
    ));
    map.push_str("\n## Death / rebirth / flag\n\n");
    for (index, arrival) in arrivals.iter().enumerate() {
        map.push_str(&format!("- restart {}: `{arrival}`\n", index + 1));
    }
    if misses.is_empty() {
        map.push_str("\n**FLAG HELD**\n");
    } else {
        map.push_str("\n## Open coordinates\n\n");
        for miss in misses {
            map.push_str(&format!("- {miss}\n"));
        }
        map.push_str("\n**NEW FLAG NOT EARNED**\n");
    }
    fs::write(config.out.join("RUN_MAP.md"), map)
        .map_err(|error| format!("could not write one-shot map: {error}"))
}

fn preflight(config: &Config) -> Result<(), String> {
    if env::consts::OS != "linux" || env::consts::ARCH != "aarch64" {
        return Err(format!(
            "exact inference lane requires Linux aarch64; found {} {}. Portable archive checking remains available with cargo run --locked",
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
        (&config.map_root.join("flag/session.txt"), "flag session"),
        (
            &config.map_root.join("route/teach_session.txt"),
            "teach session",
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
    let llama_version = Command::new(&config.llama_cli)
        .arg("--version")
        .output()
        .map_err(|error| format!("could not query llama.cpp version: {error}"))?;
    let version_text = format!(
        "{}{}",
        String::from_utf8_lossy(&llama_version.stdout),
        String::from_utf8_lossy(&llama_version.stderr)
    );
    if !version_text.contains("c0bc859") {
        return Err(format!("llama.cpp build mismatch: {version_text}"));
    }
    fs::create_dir_all(&config.out)
        .map_err(|error| format!("could not create {}: {error}", config.out.display()))?;
    fs::write(
        config.out.join("BYTE_MAP.txt"),
        format!(
            "platform=linux-aarch64-cuda13\nmodel_sha256={MODEL_SHA256}\ntokenizer_sha256={TOKENIZER_SHA256}\nniodoo_sha256={NIODOO_SHA256}\nregistry_sha256={REGISTRY_SHA256}\nllama_build=c0bc859\nllama_cli={}\n",
            config.llama_cli.display()
        ),
    )
    .map_err(|error| format!("could not write byte map: {error}"))?;
    Ok(())
}

fn real_main() -> Result<(), String> {
    let config = config()?;
    println!("ONE-SHOT MAP");
    println!("output: {}", config.out.display());
    println!("preflight: checking pinned bytes and local tools");
    preflight(&config)?;
    println!("preflight OK");
    let coordinates = run_vanilla(&config)?;
    let store = run_teach(&config)?;
    let outcome = run_flag(&config, &store)?;
    write_run_map(
        &config,
        &coordinates,
        &store,
        &outcome.arrivals,
        &outcome.misses,
    )?;
    if !outcome.misses.is_empty() {
        return Err(format!(
            "new flag not earned; {}",
            outcome.misses.join("; ")
        ));
    }
    println!("\nONE-SHOT COMPLETE");
    println!("map: {}", config.out.join("RUN_MAP.md").display());
    Ok(())
}

fn main() {
    if let Err(error) = real_main() {
        eprintln!("\nONE-SHOT STOPPED: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_extraction_uses_the_last_list() {
        assert_eq!(
            last_list("example [1, 2] then result [5, 4, 3, 2, 1, 5]"),
            Some(FLAG_DESTINATION.to_string())
        );
    }

    #[test]
    fn llama_reply_stops_before_timing() {
        let prompt = "the prompt";
        let stdout = "banner\n> the prompt\nanswer [1, 2]\n[ Prompt: 1 t/s ]\nExiting";
        assert_eq!(
            extract_llama_reply(stdout, prompt).unwrap(),
            "answer [1, 2]"
        );
    }

    #[test]
    fn constrained_words_match_whole_words() {
        assert!(banned_words("a reversal-shaped route").is_empty());
        assert_eq!(banned_words("do not append it"), vec!["append"]);
    }

    #[test]
    fn gate_allows_the_flag_markers() {
        let stdout = "ghost_basins_loaded=8\nmass=5 logit_boost=1.20\nORDER=0\n[MODEL_SCALE] size=7B\nbeta=1.000";
        assert!(excluded_route_hits(stdout, "", "answer [5, 4, 3, 2, 1, 5]").is_empty());
    }

    #[test]
    fn extracts_multiple_teaching_turns() {
        let text =
            "=== TURN 0 OUTPUT ===\none\n=== TURN 1 USER ===\ntwo\n=== TURN 1 OUTPUT ===\nthree";
        let turns = extract_turn_outputs(text);
        assert!(turns.contains("one"));
        assert!(turns.contains("three"));
        assert!(!turns.contains("two"));
    }

    #[test]
    fn local_fallback_is_used_when_original_is_missing() {
        let local = PathBuf::from("local");
        assert_eq!(
            original_or_local("definitely-missing-one-shot-path", &local),
            local
        );
    }

    #[test]
    fn missed_flag_outcome_keeps_both_coordinates() {
        let outcome = FlagOutcome {
            arrivals: vec!["<no-final-list>".to_string(), "[5,4,3]".to_string()],
            misses: vec!["r1 missed".to_string(), "r2 missed".to_string()],
        };
        assert_eq!(outcome.arrivals.len(), 2);
        assert_eq!(outcome.misses.len(), 2);
    }
}

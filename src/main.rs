use std::fs;
use std::path::{Path, PathBuf};

const GOLD: &str = "[5, 4, 3, 2, 1, 5]";
const BANNED: [&str; 8] = [
    "reverse",
    "reversed",
    "reversing",
    "backward",
    "backwards",
    "append",
    "appended",
    "appending",
];

fn read(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))
}

fn require(text: &str, needle: &str, context: &str) -> Result<(), String> {
    if text.contains(needle) {
        Ok(())
    } else {
        Err(format!("{context}: missing {needle:?}"))
    }
}

fn reject(text: &str, needle: &str, context: &str) -> Result<(), String> {
    if text
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
    {
        Err(format!("{context}: forbidden text {needle:?}"))
    } else {
        Ok(())
    }
}

fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|character: char| !character.is_ascii_alphabetic())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
}

fn check_flag_restart(root: &Path, restart: &str) -> Result<(), String> {
    let prefix = format!("flag/{restart}");
    let assistant = read(root, &format!("{prefix}/assistant.txt"))?;
    require(&assistant, GOLD, &format!("{restart} assistant"))?;
    let assistant_words: Vec<String> = words(&assistant).collect();
    for banned in BANNED {
        if assistant_words.iter().any(|word| word == banned) {
            return Err(format!("{restart} assistant: banned word {banned:?}"));
        }
    }

    let score = read(root, &format!("{prefix}/score.json"))?;
    require(&score, "\"status\": \"PASS_CONSTRAINED\"", restart)?;
    require(&score, "\"exact_answer\": true", restart)?;
    require(&score, "\"banned_hits\": []", restart)?;

    let gate = read(root, &format!("{prefix}/oracle_gate.txt"))?;
    if gate.trim() != "ORACLE_GATE_OK" {
        return Err(format!("{restart}: oracle gate was not OK"));
    }
    Ok(())
}

fn check_flag(root: &Path) -> Result<(), String> {
    check_flag_restart(root, "r1")?;
    check_flag_restart(root, "r2")?;

    let session = read(root, "flag/session.txt")?;
    for forbidden in [GOLD, "use any facts", "remember-store", "family rule"] {
        reject(&session, forbidden, "flag session")?;
    }

    let store = read(root, "flag/store_at_flag.jsonl")?;
    require(&store, "start at the end", "flag store")?;
    require(&store, "repeat the end item", "flag store")?;
    reject(&store, GOLD, "flag store")?;

    let settings = read(root, "flag/flag_settings.txt")?;
    for required in [
        "HARD_FLAG",
        "residual_ears=1",
        "dual=1",
        "mass=5",
        "beta=1.0",
        "inject=1.0",
        "posture=8",
        "logit=1.2",
        "force_all=off",
        "proc=off",
        "structural=0",
        "ORDER=0",
        "progress_tip=off",
        "runtime_profile=7b",
        "god_zone_recovery=1",
        "pin_goal=1",
    ] {
        require(&settings, required, "flag settings")?;
    }
    Ok(())
}

fn check_coordinate(
    root: &Path,
    variant: &str,
    status: &str,
    expected: &str,
    observed: &str,
) -> Result<(), String> {
    let prefix = format!("coordinates/vanilla/{variant}");
    let score = read(root, &format!("{prefix}/arc_score_assistant_only.json"))?;
    require(
        &score,
        &format!("\"status\": \"{status}\""),
        &format!("vanilla {variant} coordinate"),
    )?;
    require(
        &score,
        &format!("\"expected\": \"{expected}\""),
        &format!("vanilla {variant} coordinate"),
    )?;

    let assistant = read(root, &format!("{prefix}/assistant.txt"))?;
    require(
        &assistant,
        observed,
        &format!("vanilla {variant} coordinate output"),
    )?;
    Ok(())
}

fn check_coordinates(root: &Path) -> Result<(), String> {
    check_coordinate(root, "original", "FAIL", GOLD, "[1, 3, 4, 5, 2]")?;
    check_coordinate(root, "wording", "FAIL", GOLD, "[5, 4, 2, 1, 3]")?;
    check_coordinate(
        root,
        "short",
        "PASS_CONSTRAINED",
        "[3, 2, 1, 3]",
        "[3, 2, 1, 3]",
    )?;
    check_coordinate(
        root,
        "letters",
        "FAIL",
        "[E, D, C, B, A, E]",
        "[B, C, D, E, A, B, C, D, E]",
    )?;

    let summary = read(root, "coordinates/vanilla/BASELINE_SUMMARY.json")?;
    for required in [
        "\"arm\": \"vanilla_llamacpp\"",
        "\"model\": \"Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf\"",
        "\"system_prompt\": \"NONE\"",
        "\"temp\": 0.0",
        "\"seed\": 42",
    ] {
        require(&summary, required, "vanilla coordinate summary")?;
    }
    Ok(())
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Err(error) = check_flag(&root).and_then(|_| check_coordinates(&root)) {
        eprintln!("FAIL: {error}");
        std::process::exit(1);
    }
    println!("FLAG HELD: destination and four vanilla coordinates mapped");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_map_is_valid() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        check_flag(&root).unwrap();
        check_coordinates(&root).unwrap();
    }

    #[test]
    fn constraint_words_are_matched_as_whole_words() {
        let clean: Vec<String> = words("A reversal-shaped explanation").collect();
        assert!(!clean.iter().any(|word| BANNED.contains(&word.as_str())));

        let banned: Vec<String> = words("Do not append it").collect();
        assert!(banned.iter().any(|word| BANNED.contains(&word.as_str())));
    }
}

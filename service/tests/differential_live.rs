//! The live differential tool's command line: what it refuses before it
//! reads anything, and that it never says the token it was given.

use std::path::Path;
use std::process::Command;

const TOKEN: &str = "ghs_Quokka7731secret";

fn run(args: &[&str], token: Option<&str>) -> (Option<i32>, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_differential-live"));
    command.args(args).env_remove("GH_TOKEN");
    if let Some(token) = token {
        command.env("GH_TOKEN", token);
    }
    let output = command.output().expect("the tool runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (output.status.code(), said)
}

#[test]
fn without_a_token_nothing_is_read() {
    let synthetic = Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/synthetic");
    let (code, said) = run(&[synthetic.to_str().unwrap()], None);
    assert_eq!(code, Some(2), "{said}");
    assert!(said.contains("GH_TOKEN holds no token"), "{said}");
}

#[test]
fn a_command_that_cannot_run_never_says_the_token() {
    let empty = tempfile::tempdir().unwrap();
    let (code, said) = run(&[empty.path().to_str().unwrap()], Some(TOKEN));
    assert_eq!(code, Some(2), "{said}");
    assert!(said.contains("no recording found"), "{said}");
    let (code, said) = run(&["--budget", "lots", "x"], Some(TOKEN));
    assert_eq!(code, Some(2), "{said}");
    assert!(said.contains("--budget needs a number"), "{said}");
    let missing = empty.path().join("absent.json");
    let synthetic = Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/synthetic");
    let (code, said) = run(
        &[
            "--repositories",
            missing.to_str().unwrap(),
            synthetic.to_str().unwrap(),
        ],
        Some(TOKEN),
    );
    assert_eq!(code, Some(2), "{said}");
    assert!(said.contains("absent.json"), "{said}");
    assert!(!said.contains(TOKEN) && !said.contains("Quokka"), "{said}");
}

#[test]
fn the_tool_reaches_its_live_reads_without_panicking() {
    // Every request goes to a proxy that refuses the connection, so nothing
    // leaves this machine and each live read fails fast. What is under test is
    // that the tool gets that far: the engine's blocking thread has to be
    // started from inside the runtime, and a tool that panicked before reading
    // anything spent its whole live budget on the first repository.
    let synthetic = Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/synthetic/report");
    let registry = Path::new(env!("CARGO_MANIFEST_DIR")).join("../policies/repositories.json");
    let mut command = Command::new(env!("CARGO_BIN_EXE_differential-live"));
    command
        .args([
            "--repositories",
            registry.to_str().unwrap(),
            synthetic.to_str().unwrap(),
        ])
        .env("GH_TOKEN", TOKEN)
        .env("HTTPS_PROXY", "http://127.0.0.1:1")
        .env("https_proxy", "http://127.0.0.1:1")
        .env_remove("NO_PROXY")
        .env_remove("no_proxy");
    let output = command.output().expect("the tool runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!said.contains("panicked"), "{said}");
    assert!(matches!(output.status.code(), Some(0 | 1)), "{said}");
    assert!(!said.contains(TOKEN) && !said.contains("Quokka"), "{said}");
}

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

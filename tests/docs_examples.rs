use std::path::PathBuf;
use std::process::Command;

fn run_example(relative_path: &str) -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let example_path = manifest_dir.join(relative_path);

    let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
        .args(["file", example_path.to_str().expect("valid UTF-8 path")])
        .output()
        .expect("failed to execute rlox binary");

    assert!(
        output.status.success(),
        "example failed: {}\nstdout:\n{}\nstderr:\n{}",
        relative_path,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout should be valid UTF-8")
}

fn assert_standard_wrappers(output: &str) {
    assert!(output.contains("==== BEGIN PROGRAM OUTPUT ===="));
    assert!(output.contains("==== END PROGRAM OUTPUT ===="));
}

#[test]
fn docs_example_arithmetic() {
    let output = run_example("data/examples/01_arithmetic.rlox");
    assert_standard_wrappers(&output);
    assert!(output.contains("\n9\n"));
    assert!(output.contains("\nlox vm\n"));
}

#[test]
fn docs_example_control_flow() {
    let output = run_example("data/examples/02_control_flow.rlox");
    assert_standard_wrappers(&output);
    assert!(output.contains("\nok\n"));
    assert!(output.contains("\n3\n"));
}

#[test]
fn docs_example_functions() {
    let output = run_example("data/examples/03_functions.rlox");
    assert_standard_wrappers(&output);
    assert!(output.contains("\n25\n"));
    assert!(output.contains("\n7\n"));
}

#[test]
fn docs_example_closures() {
    let output = run_example("data/examples/04_closures.rlox");
    assert_standard_wrappers(&output);
    assert!(output.contains("\n1\n2\n1\n3\n"));
}

#[test]
fn docs_example_classes_properties() {
    let output = run_example("data/examples/05_classes_properties.rlox");
    assert_standard_wrappers(&output);
    assert!(output.contains("\n13\n"));
}

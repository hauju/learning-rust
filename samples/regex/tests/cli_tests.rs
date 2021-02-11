use assert_cmd::Command;

#[test]
fn test_program_executes_successfully() {
    let mut cmd = Command::cargo_bin("regex-sample").unwrap();
    cmd.assert().success();
}
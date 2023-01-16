use assert_cmd::Command;

#[test]

fn works() {
    let mut cmd = Command::cargo_bin("encrypt").unwrap();
    cmd.assert().failure();
    
    cmd = Command::cargo_bin("encrypt -h").unwrap();
    cmd.assert().success();
}

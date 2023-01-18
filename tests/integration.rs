use assert_cmd::Command;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("alternatives")?;

    cmd.assert().success().code(0).stdout("Hello, world!\n");

    Ok(())
}

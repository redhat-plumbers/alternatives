use assert_cmd::Command;

#[test]
fn alternatives_no_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("alternatives")?;

    cmd.assert().failure().code(2).stderr(
"Usage: alternatives [OPTIONS] <COMMAND>

Commands:
  install, --install  TODO: Help text goes here
  remove, --remove    TODO: Help text goes here
  set, --set          TODO: Help text goes here
  auto, --auto        TODO: Help text goes here
  display, --display  TODO: Help text goes here
  config, --config    TODO: Help text goes here
  help                Print this message or the help of the given subcommand(s)

Options:
      --verbose               Generate more comments about what alternatives is doing
      --quiet                 Don't generate any comments unless errors occur. This option is not yet implemented
      --test                  Don't actually do anything, just say what would be done. This option is not yet implemented
      --altdir <alt_dir>      Specifies the alternatives directory, when this is to be different from the default
      --admindir <admin_dir>  Specifies the administrative directory, when this is to be different from the default
  -h, --help                  Print help
  -V, --version               Print version
");

    Ok(())
}

#[test]
fn alternatives_install_no_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("alternatives")?;

    cmd.arg("install").assert().failure().code(2).stderr(
        "error: the following required arguments were not provided:
  <LINK>
  <NAME>
  <PATH>
  <PRIORITY>

Usage: alternatives {install|--install} <LINK> <NAME> <PATH> <PRIORITY> [INITSCRIPT]

For more information, try '--help'.
",
    );

    Ok(())
}

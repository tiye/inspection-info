use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct TopLevelInspection {
  #[argh(subcommand)]
  pub subcommand: InspectionCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum InspectionCommand {
  InIp(InspectForIp),
  CopyFile(InspectForCopyFile),
}

/// command for inspecting IP addresses.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ip")]
pub struct InspectForIp {
  #[argh(switch, short = 'd')]
  /// switch on verbose mode.
  pub detailed: bool,
}

/// command for copying files.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "cpfile")]
pub struct InspectForCopyFile {
  /// source file
  #[argh(positional)]
  pub file: String,
}

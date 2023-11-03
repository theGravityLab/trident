use clap::Args;

#[derive(Debug, Args)]
pub struct RunCommand {}

#[derive(Debug, Args)]
pub struct DeployCommand {
    #[arg(short, long)]
    pub force: bool,
    pub file: String,
}

#[derive(Debug, Args)]
pub struct InspectCommand {
    pub file: String,
}

use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum InstanceModule {
    Create(CreateCommand),
    Import,
}

#[derive(Args, Debug)]
pub struct CreateCommand {
    #[arg(short, long)]
    pub version: String,
    #[arg(short, long)]
    pub author: Option<String>,
    #[arg(short, long)]
    pub summary: Option<String>,
    pub name: String,
}

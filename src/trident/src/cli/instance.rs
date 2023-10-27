use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstanceModule {
    Create,
    Import,
}

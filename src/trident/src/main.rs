// libtrident 类似 Polymerium.Abstractions 和 Polymerium.Core
// trident 则作为命令解释器执行

use anyhow::Result;
use clap::Parser;
use machine::Machine;

use crate::cli::{CliArgs, CliModule};

mod cli;
mod io;
mod machine;

fn main() -> Result<()> {
    if let Ok(args) = if std::env::args_os().any(|f| f == "--pretty") {
        CliArgs::try_parse_from(std::env::args_os().filter(|f| f != "--pretty"))
    } else {
        CliArgs::try_parse()
    } {
        #[cfg(debug_assertions)]
        let root = std::env::current_dir()?.join(".polymerium");
        #[cfg(not(debug_assertions))]
        let root = Path::new("~/.polymerium");
        let machine = Machine::new(root);
        match args.module {
            CliModule::List => {
                let profiles = machine.scan();
                // TODO: 转成 io 特定输出
                for i in profiles{
                    println!("{}", i);
                }
            }
            _ => unimplemented!(),
        };
        Ok(())
    } else {
        Ok(())
    }
}

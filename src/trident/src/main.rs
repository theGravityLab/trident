// libtrident 类似 Polymerium.Abstractions 和 Polymerium.Core
// trident 则作为命令解释器执行

use std::io::{BufWriter, Write};
use anyhow::{Error, Result};
use clap::Parser;
use console::style;
use libtrident::machine::Machine;

use crate::cli::{CliArgs, CliModule};
use crate::cli::instance::InstanceModule;

mod cli;
mod io;

fn main() {
    // pretty 输出规范：无论如何都不会异常结束，异常结束意味着程序错误而非输入错误。
    // 无论任何操作都有可能得到多条    // 可能包含错误的输出，对于只需要结果的（例如 trident inspect）要求在进
    // 程正常退出后检查所有输出中是否有需要    // 的结果，如果没有则检查最后一条错误。
    // 对于需要过程的，在进程退出之前，都可以即时捕获而非缓存输出并处理。
    let pretty: bool;
    if let Ok(args) = if std::env::args_os().any(|f| f == "--pretty") {
        pretty = true;
        CliArgs::try_parse_from(std::env::args_os().filter(|f| f != "--pretty"))
    } else {
        pretty = false;
        CliArgs::try_parse()
    } {
        #[cfg(debug_assertions)]
            let root = std::env::current_dir().unwrap().join(".polymerium");
        #[cfg(not(debug_assertions))]
            let root = Path::new("~/.polymerium");
        let machine = Machine::new(root);
        if let Err(err) = process(machine, args, pretty) {
            // TODO: write error in two modes
            println!("{:?}", err);
        }
    } else {
        // TODO: write error in two modes
        println!("parse failed");
    }
}

fn process(machine: Machine, args: CliArgs, pretty: bool) -> Result<()> {
    match args.module {
        CliModule::Inspect(it) => {
            let profile = machine.load_profile(&it.file)?;
            if pretty {
                todo!()
            } else {
                let mut buf = BufWriter::new(std::io::stdout());
                writeln!(buf, "{}{}", style(&profile.name).yellow().bold(), style(format!("({})", &profile.author)).dim())?;
                writeln!(buf, "{}", style(&profile.summary))?;
                writeln!(buf, "{}", style("Components:").blue().bold())?;
                for c in &profile.metadata.components {
                    writeln!(buf, "{:>24} {}", c.id, style(&c.version).dim())?;
                }
                writeln!(buf, "{}", style("Attachments:").blue().bold())?;
                for l in &profile.metadata.attachments {
                    writeln!(buf, "{}{}{}", style("Layer(").cyan(), style(format!("\"{}\"", &l.summary)).green(), style(format!("{}{})", if l.from == profile.reference { ",🔒" } else { "" }, if !l.enabled { ",🚫" } else { "" })).cyan())?;
                    for a in &l.content {
                        writeln!(buf, "{}", if l.enabled { style(a.as_str()) } else { style(a.as_str()).strikethrough() })?;
                    }
                }
                buf.flush()?;
            }
            Ok(())
        }
        CliModule::Instance(it) => {
            match it {
                InstanceModule::Create(create) => {
                    let profile = machine.create_profile(&create.name, create.author.as_deref(), create.summary.as_deref(), create.version.as_deref())?;
                    if pretty {
                        todo!()
                    } else {
                        println!("Create {} at {}.ron", &create.name, &create.name);
                        Ok(())
                    }
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

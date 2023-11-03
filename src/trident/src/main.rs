// libtrident 类似 Polymerium.Abstractions 和 Polymerium.Core
// trident 则作为命令解释器执行

use anyhow::Result;
use clap::Parser;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use libtrident::machine::Machine;
use std::io::{BufWriter, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::cli::instance::InstanceModule;
use crate::cli::{CliArgs, CliModule};

mod cli;
mod io;
mod repo;

// 导入整合包的过程中例外文件存放在 storage 中的子目录，并会自动添加一个 io.github.trident.storage 到组件中，
// 该组件的 version 即为子目录名。除了由导入自动添加的 storage 目录，也可以用户自己创建并手动添加该组件。
fn main() {
    // pretty 输出规范：无论如何都不会异常结束，异常结束意味着程序错误而非输入错误。
    // 无论任何操作都有可能得到多条可能包含错误的输出，对于只需要结果的（例如 trident inspect）要求在进程正常退
    // 出后检查所有输出中是否有需要的结果，如果没有则检查最后一条错误。
    // 对于需要过程的，在进程退出之前，都可以即时捕获而非缓存输出并处理。

    // 原则上 trident 对 .trident 目录拥有所有权，Polymerium 不应该访问 .trident 来确定信息，所有对文件的操
    // 做都应该借助 trident 完成。
    let pretty: bool;
    match if std::env::args_os().any(|f| f == "--pretty") {
        pretty = true;
        CliArgs::try_parse_from(std::env::args_os().filter(|f| f != "--pretty"))
    } else {
        pretty = false;
        CliArgs::try_parse()
    } {
        Ok(args) => {
            #[cfg(debug_assertions)]
            let root = std::env::current_dir().unwrap().join(".trident");
            #[cfg(not(debug_assertions))]
            let root = Path::new("~/.trident");
            let machine = Machine::new(root);
            if let Err(err) = process(machine, args, pretty) {
                // TODO: write error in two modes
                println!("{:?}", err);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
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
                writeln!(
                    buf,
                    "{}{}",
                    style(&profile.name).yellow().bold(),
                    style(format!("({})", &profile.author)).dim()
                )?;
                writeln!(buf, "{}", style(&profile.summary))?;
                writeln!(buf, "{}", style("Components:").blue().bold())?;
                for c in &profile.metadata.components {
                    writeln!(buf, "{:>24} {}", c.id, style(&c.version).dim())?;
                }
                writeln!(buf, "{}", style("Attachments:").blue().bold())?;
                for l in &profile.metadata.attachments {
                    writeln!(
                        buf,
                        "{}{}{}",
                        style("Layer(").cyan(),
                        style(format!("\"{}\"", &l.summary)).green(),
                        style(format!(
                            "{}{})",
                            if l.from == profile.reference {
                                ",🔒"
                            } else {
                                ""
                            },
                            if !l.enabled { ",🚫" } else { "" }
                        ))
                        .cyan()
                    )?;
                    for a in &l.content {
                        writeln!(
                            buf,
                            "{}",
                            if l.enabled {
                                style(a.as_str())
                            } else {
                                style(a.as_str()).strikethrough()
                            }
                        )?;
                    }
                }
                buf.flush()?;
            }
            Ok(())
        }
        CliModule::Instance(it) => match it {
            InstanceModule::Create(create) => {
                let _ = machine.create_profile(
                    &create.name,
                    create.author.as_deref(),
                    create.summary.as_deref(),
                    create.version.as_deref(),
                )?;
                if pretty {
                    todo!()
                } else {
                    println!(
                        "Create {} at {}.ron",
                        style(format!("\"{}\"", &create.name)).green(),
                        &create.name
                    );
                    Ok(())
                }
            }
            _ => unimplemented!(),
        },
        CliModule::Deploy(it) => {
            let deploy = machine.deploy(&it.file)?;
            let bar = ProgressBar::new_spinner();
            bar.set_style(ProgressStyle::with_template("{spinner:.magenta} {msg}").unwrap());
            bar.set_message("Check polylock status");
            bar.enable_steady_tick(Duration::from_millis(100));
            sleep(Duration::from_secs(5));
            bar.finish();
            Ok(())
        }
        _ => unimplemented!(),
    }
}

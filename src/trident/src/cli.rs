// 针对实例和本地文件的管理
// trident run $:instance
// trident clean $:instance
// trident deploy $:instance
// trident remove $:instance
// trident display $:instance
// trident list
// 针对 instance 管理
// trident instance create --version {}
// trident instance import
// 针对某个 instance 的元数据管理(M=component,attachment)
// trident M add --instance {} $:[poly-res]
// trident M remove --instance {} $:[poly-res]
// trident M list --instance {}
// trident attachment enable/disable --instance {} $:poly-res
// 在线数据源仓库管理
// trident repository list
// 在线资源查询
// trident resource search --repository {} --take {} --skip --filters {{}} {} $:keyword
// trident resource resolve $:poly-res

use clap::{Parser, Subcommand};

use self::{generic::RunCommand, instance::InstanceModule};

pub mod generic;
pub mod instance;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub module: CliModule,
}

#[derive(Debug, Subcommand)]
pub enum CliModule {
    List,
    Run(RunCommand),
    Deploy,
    Clean,
    Remove,
    #[clap(subcommand)]
    Instance(InstanceModule),
}

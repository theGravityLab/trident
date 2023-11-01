// 针对实例和本地文件的管理
// trident list
// trident run $:instance
// trident inspect $:instance
// trident clean $:instance
// trident deploy $:instance
// trident remove $:instance
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

// deploy
// 第一步 Flatten，根据 Metadata 构建出 Polylock
// 这一步需要解析所有资源文件和依赖并导出，以便下次可以直接仅检查文件完整性即可启动游戏
// 第二步 Restore，即检查并补全文件到 Polylock 状态

use crate::cli::generic::{DeployCommand, InspectCommand};
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
    Inspect(InspectCommand),
    Run(RunCommand),
    Deploy(DeployCommand),
    Clean,
    Remove,
    #[clap(subcommand)]
    Instance(InstanceModule),
}

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Mount point path
    #[clap(required = true)]
    pub mount_point: String,

    /// Automatically unmount on process exit
    #[clap(long, short)]
    pub unmount_at_exit: bool,

    /// Allow root user to access filesystem
    #[clap(long, short)]
    pub root_access: bool,
}
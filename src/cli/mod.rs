use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Mount point path
    #[clap(required = true)]
    pub mount_point: String,

    /// Allow root user to access filesystem
    #[clap(long, short)]
    pub root_access: bool,

    /// Unique ID for FSTree identification
    #[clap(long, short)]
    pub id: String,
}
mod http;
mod sdk;
mod cli;
mod core;
mod mirroring;

use clap::Parser;
use fuser::MountOption;

fn main() {
    let args = cli::Args::parse();
    let mut options = vec![MountOption::RW, MountOption::FSName("fs0x".to_string())];

    if args.unmount_at_exit {
        options.push(MountOption::AutoUnmount);
    }

    if args.root_access {
        options.push(MountOption::AllowRoot);
    }

    fuser::mount2(core::FS0X::new(), args.mount_point, &options).unwrap();
}
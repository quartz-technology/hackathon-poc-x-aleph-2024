mod http;
mod sdk;
mod cli;
mod core;
mod mirroring;

use clap::Parser;
use fuser::MountOption;
use sdk::AlephSDK;

fn main() {
    let args = cli::Args::parse();
    let mut options = vec![MountOption::RW, MountOption::FSName("fs0x".to_string())];

    if args.unmount_at_exit {
        options.push(MountOption::AutoUnmount);
    }

    if args.root_access {
        options.push(MountOption::AllowRoot);
    }

    let client = http::HttpClient::new().unwrap();
    let sdk = sdk::AlephSDK::new(client);

    let fs0x = core::FS0X::new(sdk);

    fuser::mount2(fs0x, args.mount_point, &options).unwrap();
}
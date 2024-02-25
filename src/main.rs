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
    let mut options = vec![
        MountOption::RW,
        MountOption::AutoUnmount,
        MountOption::FSName("fs0x".to_string()),
    ];

    if args.root_access {
        options.push(MountOption::AllowRoot);
    }

    let client = http::HttpClient::new().unwrap();
    let sdk = sdk::AlephSDK::new(client);

    let fs0x = core::FS0X::new(sdk, args.id);

    fuser::mount2(fs0x, args.mount_point, &options).unwrap();
}
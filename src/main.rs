use std::{env, io, path::PathBuf};

use clap::Parser;
use fuser::MountOption;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use drive::{model::Credentials,AlistDrive, DriveConfig};
use vfs::AlistDriveFileSystem;
use tracing::{debug, error, info, warn};



mod drive;
mod error;
mod file_cache;
mod vfs;
mod cache;

#[derive(Parser, Debug)]
#[clap(name = "alist-fuse", about, version, author)]
struct Opt {
    /// Mount point
    #[clap(parse(from_os_str))]
    path: PathBuf,

    #[structopt(long, env = "Alist_USER")]
    alist_user: String,

    #[structopt(long, env = "Alist_PASSWORD")]
    alist_password: String,

    #[structopt(long, env = "API_URL", default_value = "")]
    api_url: String,

    /// Working directory, refresh_token will be stored in there if specified
    #[clap(short = 'w', long)]
    workdir: Option<PathBuf>,
    /// alist PDS domain id
    #[clap(long)]
    domain_id: Option<String>,
    /// Allow other users to access the drive
    #[clap(long)]
    allow_other: bool,
    /// Read/download buffer size in bytes, defaults to 10MB
    #[clap(short = 'S', long, default_value = "10485760")]
    read_buffer_size: usize,

    /// Upload buffer size in bytes, defaults to 16MB
    #[clap(long, default_value = "16777216")]
    upload_buffer_size: usize,
}

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "native-tls-vendored")]
    openssl_probe::init_ssl_cert_env_vars();

    tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_env("ALIST_FUSE_LOG"))
    .init();
    if env::var("ALIST_FUSE_LOG").is_err() {
       env::set_var("ALIST_FUSE_LOG", "alist_fuse=info");
    }

    let opt = Opt::parse();
    let drive_config = if opt.api_url.is_empty() {
        DriveConfig {
            api_base_url: opt.api_url.clone(),
            refresh_token_url: format!("{}/api/auth/login/hash",opt.api_url.clone()),
            workdir: opt.workdir,
        }
    } else {
        DriveConfig {
            api_base_url: opt.api_url.clone(),
            refresh_token_url: format!("{}/api/auth/login/hash",opt.api_url.clone()),
            workdir: opt.workdir,
        }
    };

   

    let credentials = Credentials{
        username:opt.alist_user,
        password:opt.alist_password,
    };


    let drive = AlistDrive::new(drive_config,credentials).map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "initialize alistDrive client failed")
    })?;

    let _nick_name = drive.nick_name.clone();
    let vfs = AlistDriveFileSystem::new(drive, opt.read_buffer_size,opt.upload_buffer_size);
    let mut mount_options = vec![MountOption::AutoUnmount, MountOption::NoAtime];
    // read only for now
    // mount_options.push(MountOption::RO);
    if opt.allow_other {
        mount_options.push(MountOption::AllowOther);
    }
    if cfg!(target_os = "macos") {
        mount_options.push(MountOption::CUSTOM("local".to_string()));
        mount_options.push(MountOption::CUSTOM("noappledouble".to_string()));
        let volname = if let Some(nick_name) = _nick_name {
            format!("volname=Alist网盘({})", nick_name)
        } else {
            "volname=Alist网盘".to_string()
        };
        mount_options.push(MountOption::CUSTOM(volname));
    }
    fuser::mount2(vfs, opt.path, &mount_options)?;
    Ok(())
}

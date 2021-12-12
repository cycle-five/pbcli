use url::Url;
use crate::PasteFormat;
use clap::{Parser};

const ABOUT: &str =
    "pbcli is a command line client which allows to upload and download
pastes from privatebin directly from the command line.

Project home page: https://github.com/Mydayyy/pbcli";

#[derive(Debug, Parser, Clone)]
#[clap(setting = clap::AppSettings::AllArgsOverrideSelf, version = env ! ("CARGO_PKG_VERSION"), author = "Mydayyy <dev@mydayyy.eu>", about = ABOUT)]
#[clap(rename_all = "kebab-case")]
pub struct Opts {
    #[clap(required_unless_present("host"), parse(try_from_str))]
    pub url: Option<Url>,

    #[clap(long, parse(try_from_str))]
    pub host: Option<Url>,

    #[clap(long, arg_enum, default_value = "plaintext")]
    pub format: PasteFormat,

    #[clap(long, default_value = "1week")]
    pub expire: String,

    #[clap(long)]
    pub json: bool,
    #[clap(long, conflicts_with = "discussion")]
    pub burn: bool,
    #[clap(long)]
    pub discussion: bool,

    #[clap(long, parse(from_os_str), value_name = "FILE")]
    pub download: Option<std::path::PathBuf>,
    #[clap(long)]
    pub overwrite: bool,
    // #[clap(long)]
    // skip_extension: bool,

    #[clap(long, parse(from_os_str), value_name = "FILE")]
    pub upload: Option<std::path::PathBuf>,

    #[clap(long)]
    pub password: Option<String>,

    #[clap(long, requires_all(&["oidc-client-id", "oidc-username", "oidc-password"]))]
    pub oidc_token_url: Option<String>,

    #[clap(long)]
    pub oidc_client_id: Option<String>,

    #[clap(long)]
    pub oidc_username: Option<String>,

    #[clap(long)]
    pub oidc_password: Option<String>,
}

impl Opts {
    pub fn get_url(&self) -> &Url {
        self.url.as_ref().unwrap_or_else(|| self.host.as_ref().unwrap())
    }
}
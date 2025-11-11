use directories::ProjectDirs;
use std::{
    ffi::OsStr,
    fmt::Display,
    net::{self, Ipv4Addr, Ipv6Addr},
    path::{Path, PathBuf},
    process::Command,
};
use which::which;

#[cfg(test)]
mod tests;

const EXECUTABLE_NAME: &str = "prismlauncher";
// #[cfg(target_os = "linux")]
// const EXECUTABLE_NAME_FLATPAK: &str = "org.prismlauncher.PrismLauncher";

#[derive(Debug, Clone)]
pub enum Hostname {
    Dns(String),
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}

impl Display for Hostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hostname::Dns(string) => string.clone(),
                Hostname::Ipv4(ipv4_addr) => ipv4_addr.to_string(),
                Hostname::Ipv6(ipv6_addr) => ipv6_addr.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum GameMode {
    SinglePlayer { world_name: String },
    MultiPlayer(Address),
}

#[derive(Debug, Clone)]
pub struct Address {
    pub hostname: Hostname,
    pub port: u16,
}

impl Default for Address {
    fn default() -> Self {
        Address {
            hostname: Hostname::Dns("localhost".to_string()),
            port: 25565,
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hostname.to_string(), self.port)
    }
}

pub struct LaunchParameters {
    /// Folder name in instance folder.
    instance_id: String,
    gamemode: Option<GameMode>,
    /// Account name, otherwise let the launcher decide the default account.
    profile: Option<String>,
}

pub fn data_directory() -> String {
    execute(&["--get-directory"])
}

pub fn list_accounts() -> Vec<String> {
    let output = execute(&["--list-accounts"]);
    output
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn launch(parameters: LaunchParameters) -> String {
    let mut args = vec![];

    args.push("--launch".to_string());
    args.push(parameters.instance_id);

    if let Some(gamemode) = parameters.gamemode {
        match gamemode {
            GameMode::SinglePlayer { world_name } => {
                // profile shouldn't matter since its single player.
                args.push("--world".to_string());
                args.push(world_name);
            }
            GameMode::MultiPlayer(address) => {
                args.push("--server".to_string());
                let cmd = format!("{}:{}", address.hostname.to_string(), address.port);
                args.push(cmd);
            }
        }
    }

    if let Some(profile) = parameters.profile {
        args.push("--profile".to_string());
        args.push(profile);
    }

    execute(&args)
}

pub fn version() -> String {
    let string = execute(&["--version"]);
    string.split("PrismLauncher ").collect()
}

fn execute(args: &[impl AsRef<OsStr>]) -> String {
    let executable = which(EXECUTABLE_NAME).expect("Failed to locate PrismLauncher.");

    let output = Command::new(executable)
        .args(args)
        .output()
        .expect("PrismLauncher command failed.");

    String::from_utf8(output.stdout).expect("Failed to encode to utf8.")
}

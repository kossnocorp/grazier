use notify_debouncer_full::DebouncedEvent;
use std::{fs::read, path::PathBuf};
use tokio::fs::File;

const PNPM_LOCK: &str = "pnpm-lock.yaml";

pub enum NpmFlavor {
    Pnpm(PnpmFlavor),
    Npm(PlainFlavor),
}

pub async fn find_npm_flavor(path: &PathBuf) -> Result<NpmFlavor, Box<dyn std::error::Error>> {
    // Probe pnpm
    let pnpm_lock = File::open(path.join(PNPM_LOCK)).await;
    if let Ok(_) = pnpm_lock {
        println!("Detected pnpm!");
        return Ok(NpmFlavor::Pnpm(PnpmFlavor {}));
    }

    // Try reading package-lock.json
    println!("Detected npm!");
    Ok(NpmFlavor::Npm(PlainFlavor {}))
}

pub fn match_npm_flavor_event(path: &PathBuf, event: Option<&DebouncedEvent>) -> bool {
    match event {
        Some(event) => {
            let pnpm_lock = path.join(PNPM_LOCK);
            if pnpm_lock.exists() {
                return true;
            }
        }
        _ => {}
    }
    false
}

// pub fn load_workspaces_list(flavor: FlavorType) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     match flavor {
//         FlavorType::Pnpm => Ok(vec![]),
//         FlavorType::Npm => Ok(vec![]),
//     }
// }

pub struct PlainFlavor {}

pub struct PnpmFlavor {}

// impl Flavor for PnpmFlavor {
//     async fn load_workspaces_list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         Ok(vec![])
//     }
// }

// pub struct NpmFlavor {}

// impl Flavor for NpmFlavor {
//     async fn load_workspaces_list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         Ok(vec![])
//     }
// }

// pub trait Flavor {
//     async fn load_workspaces_list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
// }

// pub enum FlavorState {
//     Initial,
//     Loading,
//     Errored,
//     Ready,
// }

use std::{fs::read, path::PathBuf};
use tokio::fs::File;

const PNPM_LOCK: &str = "pnpm-lock.yaml";

pub enum Flavor {
    Pnpm(PnpmFlavor),
    Npm(NpmFlavor),
}

pub async fn find_npm_flavor(path: &PathBuf) -> Result<Flavor, Box<dyn std::error::Error>> {
    // Probe pnpm
    let pnpm_lock = File::open(path.join(PNPM_LOCK)).await;
    if let Ok(_) = pnpm_lock {
        println!("Detected pnpm!");
        return Ok(Flavor::Pnpm(PnpmFlavor {}));
    }

    // Try reading package-lock.json
    println!("Detected npm!");
    Ok(Flavor::Npm(NpmFlavor {}))
}

pub struct PnpmFlavor {}

pub struct NpmFlavor {}

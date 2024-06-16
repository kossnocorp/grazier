use std::path::{Path, PathBuf};

pub enum NpmFlavor {
    Npm,
    Pnpm,
    // [TODO]
    // Yarn,
    // Bun,
}

const NPM_PACKAGE_JSON_NAME: &str = "package.json";

struct NpmPackageJson {
    name: String,
    version: Option<String>,
    description: Option<String>,
}

pub struct NpmPackage {
    path: PathBuf,
    state: Option<Box<dyn State>>,
}

impl NpmPackage {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: Some(Box::new(Loading {})),
        }
    }

    pub async fn load(&mut self) {
        let path = self.path.clone().push(NPM_PACKAGE_JSON_NAME);
    }

    // pub fn set_state(&mut self, state: Box<dyn State>) {
    //     self.state = Some(state);
    // }

    // pub fn state(&self) -> &Option<Box<dyn State>> {
    //     &self.state
    // }
}

trait State {}

struct Loading {}

impl State for Loading {}

struct Errored {}

impl State for Errored {}

struct Ready {}

impl State for Ready {}

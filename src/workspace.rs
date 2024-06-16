use std::path::PathBuf;

use crate::npm_package::NpmPackage;

pub struct Workspace {
    path: PathBuf,
    state: Option<Box<dyn State>>,
    npm_package: Option<NpmPackage>,
}

impl Workspace {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: Some(Box::new(Loading {})),
            npm_package: None,
        }
    }

    async fn load(&mut self) {
        // let path = self.path.clone().push(JSON_NAME);
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

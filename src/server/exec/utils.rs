use std::path::PathBuf;
#[derive(Default, Debug)]
pub struct Utils {
    home: PathBuf,
}

impl Utils {
    pub fn new() -> Self {
        Self {
            home: dirs::home_dir().unwrap(),
        }
    }
    pub fn replace_home(&self, path: &str) -> String {
        path.replacen('~', self.home.to_str().unwrap(), 1)
    }
}

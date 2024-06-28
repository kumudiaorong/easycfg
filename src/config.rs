mod dis;
mod os_rel;

use anyhow::Result;
pub use dis::Distri;
pub use os_rel::get_release;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use xcfg::File as XFile;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Package {
    pub distri: Distri,
    pub install: Option<Vec<String>>,
    pub remove: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub path: Option<String>,
    pub pkg: Option<Vec<Package>>,
    pub commands: Option<Vec<String>>,
    pub slink: Option<Vec<(String, String)>>,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LinuxConfig {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub linux: Option<LinuxConfig>,
}

pub type Cfg = LinuxConfig;

pub fn init(dir: String) -> Result<(Cfg, PathBuf)> {
    let home = dirs::home_dir().ok_or(IoError::new(
        IoErrorKind::Other,
        "Could not find home directory",
    ))?;
    let cwd = match dir.strip_prefix("~/") {
        Some(p) => home.join(p),
        None => PathBuf::from(dir),
    };
    let cwd = std::fs::canonicalize(cwd)?;

    let cfg_dir = cwd.join("tasks.toml");
    let mut xf = XFile::<Config>::default().path(cfg_dir.to_str().unwrap());
    xf.load().unwrap();

    let mut tasks = {
        #[cfg(target_os = "linux")]
        {
            xf.inner.linux.unwrap_or_default()
        }
    };
    for task in &mut tasks.tasks {
        let src_parent = cwd.join(match &task.path {
            Some(path) => path,
            None => "",
        });
        if let Some(slinks) = &mut task.slink {
            for (src, dst) in slinks {
                *src = src_parent.join(src.clone()).to_string_lossy().to_string();
                if let Some(rest) = dst.strip_prefix("~/") {
                    *dst = home.join(rest).to_string_lossy().to_string();
                }
            }
        }
    }
    Ok((tasks, cwd))
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, remove_dir_all},
        panic,
    };

    use super::*;

    #[test]
    fn test_init() {
        let testdir = "unit_config_test";
        let hook = std::panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            remove_dir_all(testdir).unwrap();
            hook(info);
        }));
        fs::create_dir(testdir).unwrap();
        let path = PathBuf::from(testdir).join("tasks.toml");
        fs::write(
            path,
            r#"
            [linux]
            [[linux.tasks]]
            name = "test"
            [[linux.tasks.pkg]]
            distri = "Arch"
            install = ["test"]
            remove = ["test"]
            [[linux.tasks]]
            name = "test2"
            [[linux.tasks.pkg]]
            distri = "Arch"
            install = ["test"]
            remove = ["test"]
        "#,
        )
        .unwrap();
        // let cfg = init(PathBuf::from(testdir));
        // assert_eq!(cfg.tasks.len(), 2);
        // assert_eq!(cfg.tasks[0].name, "test");
        // assert_eq!(cfg.tasks[0].pkg.as_ref().unwrap()[0].distri, Distri::Arch);
        // assert_eq!(
        //     cfg.tasks[0].pkg.as_ref().unwrap()[0]
        //         .install
        //         .as_ref()
        //         .unwrap()[0],
        //     "test"
        // );
        // assert_eq!(
        //     cfg.tasks[0].pkg.as_ref().unwrap()[0]
        //         .remove
        //         .as_ref()
        //         .unwrap()[0],
        //     "test"
        // );
        // assert_eq!(cfg.tasks[1].name, "test2");
        // assert_eq!(cfg.tasks[1].pkg.as_ref().unwrap()[0].distri, Distri::Arch);
        // assert_eq!(
        //     cfg.tasks[1].pkg.as_ref().unwrap()[0]
        //         .install
        //         .as_ref()
        //         .unwrap()[0],
        //     "test"
        // );
        // assert_eq!(
        //     cfg.tasks[1].pkg.as_ref().unwrap()[0]
        //         .remove
        //         .as_ref()
        //         .unwrap()[0],
        //     "test"
        // );
        // remove_dir_all(testdir).unwrap();
    }
    #[test]
    fn test_example() {
        let testdir = "unit_config_test";
        let hook = std::panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            remove_dir_all(testdir).unwrap();
            hook(info);
        }));
        fs::create_dir(testdir).unwrap();
        let path = PathBuf::from(testdir).join("tasks.yaml");
        let cfg = Config {
            linux: Some(LinuxConfig {
                tasks: vec![
                    Task {
                        name: "test".to_string(),
                        path: Some("test".to_string()),
                        pkg: Some(vec![Package {
                            distri: Distri::Arch,
                            install: Some(vec!["test".to_string()]),
                            remove: Some(vec!["test".to_string()]),
                        }]),
                        commands: None,
                        slink: None,
                    },
                    Task {
                        name: "test2".to_string(),
                        path: None,
                        pkg: Some(vec![Package {
                            distri: Distri::Arch,
                            install: Some(vec!["test".to_string()]),
                            remove: Some(vec!["test".to_string()]),
                        }]),
                        commands: Some(vec!["test".to_string()]),
                        slink: Some(vec![("test".to_string(), "test".to_string())]),
                    },
                ],
            }),
        };
        let mut xf = XFile::default().path(path.to_str().unwrap());
        xf.inner = cfg;
        xf.save().unwrap();
    }
}

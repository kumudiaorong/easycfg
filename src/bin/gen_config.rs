use std::{fs, path::PathBuf};
fn main() {
    use easycfg::config::*;
    let testdir = "unit_config_test";
    if let Err(e) = fs::create_dir(testdir) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("create dir error: {}", e);
        }
    }
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
    let mut xf = xcfg::File::default().path(path.to_str().unwrap());
    xf.inner = cfg;
    xf.save().unwrap();
}

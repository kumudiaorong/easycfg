use crate::config::Distri;

#[derive(Default, Debug)]
struct Command {
    pub exec: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
}

#[derive(Default, Debug)]
pub struct DistriOpt {
    pub distri: Distri,
    pub install: Command,
}

impl From<Distri> for DistriOpt {
    fn from(distri: Distri) -> Self {
        match distri {
            Distri::Unknown => DistriOpt {
                distri,
                install: Command {
                    exec: "".to_string(),
                    args: vec![],
                    stdin: None,
                },
            },
            Distri::Arch => DistriOpt {
                distri,
                install: Command {
                    exec: "pacman".to_string(),
                    args: vec!["-S".to_string()],
                    stdin: Some("\n".to_string()),
                },
            },
            Distri::OpenSUSE => DistriOpt {
                distri,
                install: Command {
                    exec: "zypper".to_string(),
                    args: vec!["install".to_string()],
                    stdin: Some("\n".to_string()),
                },
            },
        }
    }
}

mod exec;
use anyhow::Result;

use super::config::get_release;
use super::config::Cfg;
use super::config::Distri;
use std::io::Write;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Result as IoResult};
use std::process::Output;

// const ROOT_SHELL: [&str; 1] = ["bash"];

#[derive(Default, Debug)]
pub struct ServerBuilder {
    pub distri: Distri,
    root: Option<String>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            distri: get_release(),
            root: None,
        }
    }
    fn check_root(&mut self, cfg: &Cfg) -> Result<()> {
        let mut need_root = false;
        for task in &cfg.tasks {
            if let Some(pkgs) = &task.pkg {
                need_root = pkgs.iter().any(|p| p.distri == self.distri);
                if need_root {
                    break;
                }
            }
        }
        if need_root {
            print!("Enter root password: ");
            std::io::stdout().flush()?;
            let mut pass = String::new();
            std::io::stdin().read_line(&mut pass)?;
            self.root = Some(pass.trim().to_string());
        }
        Ok(())
    }
    pub fn build(mut self, cfg: Cfg) -> Result<Server> {
        self.check_root(&cfg)?;
        Ok(Server::new(self.distri, cfg))
    }
}

#[derive(Default, Debug)]
pub struct Server {
    pub cfg: Cfg,
    exec: exec::Executer,
}

impl Server {
    pub fn new(distri: Distri, cfg: Cfg) -> Self {
        Self {
            cfg,
            exec: exec::Executer::new(distri),
        }
    }
    pub fn exec(&mut self, name: &str) -> (Vec<Output>, IoResult<()>) {
        let task = match self.cfg.tasks.iter().find(|t| t.name == name) {
            Some(task) => task,
            None => {
                return (
                    Vec::new(),
                    Err(IoError::new(IoErrorKind::NotFound, "Name error")),
                )
            }
        };
        self.exec.exec(task)
    }
    pub fn exec_by_index(&mut self, index: usize) -> (Vec<Output>, IoResult<()>) {
        if index >= self.cfg.tasks.len() {
            return (
                Vec::new(),
                Err(IoError::new(IoErrorKind::NotFound, "Index error")),
            );
        }
        let task = &self.cfg.tasks[index];
        self.exec.exec(task)
    }
    pub fn exec_all(&mut self) -> (Vec<Output>, IoResult<()>) {
        let mut outputs = Vec::new();
        for task in &self.cfg.tasks {
            let (output, result) = self.exec.exec(task);
            outputs.extend(output);
            if result.is_err() {
                return (outputs, result);
            }
        }
        (outputs, Ok(()))
    }
}

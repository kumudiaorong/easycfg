mod pkg;
use crate::config::Task;
use pkg::DistriOpt;
use std::io::{ErrorKind as IoErrorKind, Result as IoResult};
use std::path::Path;
use std::path::PathBuf;
use std::process::Output;
#[derive(Default, Debug)]
pub struct Executer {
    distri: DistriOpt,
    current_dir: PathBuf,
}

impl Executer {
    pub fn new(distri: crate::config::Distri, current_dir: PathBuf) -> Self {
        Self {
            distri: distri.into(),
            current_dir,
        }
    }
    fn exec_cmd(&self, cmd: &str, current_dir: PathBuf) -> IoResult<Output> {
        std::process::Command::new("sh")
            .current_dir(current_dir)
            .args(["-c", cmd])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
    }
    fn symlink(&mut self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> IoResult<()> {
        loop {
            let result = {
                #[cfg(unix)]
                {
                    std::os::unix::fs::symlink(&src, &dst)
                }
                #[cfg(windows)]
                {
                    std::os::windows::fs::symlink_file(src, dest)
                }
            };
            // println!("first result: {:?}", result);
            let Err(err) = result else {
                break Ok(());
            };
            match err.kind() {
                IoErrorKind::AlreadyExists => {
                    std::fs::remove_file(&dst).or_else(|_| std::fs::remove_dir_all(&dst))?;
                }
                IoErrorKind::NotFound => {
                    // let a=IoErrorKind::IsADirectory;
                    let _ = dst
                        .as_ref()
                        .parent()
                        .ok_or(err)
                        .map(std::fs::create_dir_all)?;
                }
                _ => break Err(err),
            };
        }
    }
    fn pkg_apply(&mut self, pkg: &crate::config::Package) -> IoResult<()> {
        todo!()
    }

    pub fn exec(&mut self, task: &Task) -> (Vec<Output>, IoResult<()>) {
        let mut outputs = Vec::new();
        if let Some(symlinks) = &task.slink {
            for (src, dst) in symlinks {
                let mut output = Output {
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                    status: std::process::ExitStatus::default(),
                };
                match self.symlink(src, dst) {
                    Ok(_) => {
                        output.stdout = format!("[symlink] {} -> {}", src, dst).into_bytes();
                    }
                    Err(e) => {
                        output.stderr = format!("[symlink] {} -> {}: {}", src, dst, e).into_bytes();
                        outputs.push(output);
                        return (outputs, Err(e));
                    }
                }
                // todo!()
                // status: std::process::ExitStatus::default(),
                outputs.push(output);
            }
        }
        if let Some(commands) = &task.commands {
            for cmd in commands {
                let start = format!("[exec] {}\n", cmd).into_bytes();
                let mut output = Output {
                    stdout: start.clone(),
                    stderr: Vec::new(),
                    status: std::process::ExitStatus::default(),
                };
                match self.exec_cmd(
                    cmd,
                    self.current_dir
                        .join(task.path.as_deref().unwrap_or(task.name.as_str())),
                ) {
                    Ok(exec_output) => {
                        output.stdout.extend(exec_output.stdout.into_iter());
                        if !exec_output.stderr.is_empty() {
                            output.stderr.extend(start);
                            output.stderr.extend(exec_output.stderr.into_iter());
                        }
                        outputs.push(output);
                    }
                    Err(e) => {
                        output.stderr.extend(start);
                        outputs.push(output);
                        return (outputs, Err(e));
                    }
                }
            }
        }
        // if let Some(pkgs) = &task.pkg {
        //     for pkg in pkgs {
        //         if pkg.distri != self.distri.distri {
        //             continue;
        //         }
        //         let mut output = Output {
        //             stdout: Vec::new(),
        //             stderr: Vec::new(),
        //             status: std::process::ExitStatus::default(),
        //         };
        //         match self.pkg_apply(pkg) {
        //             Ok(_) => {
        //                 output.stdout = format!("[pkg] {:?}\n", pkg).into_bytes();
        //             }
        //             Err(e) => {
        //                 output.stderr = format!("[pkg] {:?}: {}\n", pkg, e).into_bytes();
        //                 outputs.push(output);
        //                 return (outputs, Err(e));
        //             }
        //         }
        //         outputs.push(output);
        //     }
        // }
        (outputs, Ok(()))
    }
}

#[cfg(test)]
mod test;

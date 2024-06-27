mod pkg;
use crate::config::Task;
use pkg::DistriOpt;
use std::io::{ErrorKind as IoErrorKind, Result as IoResult};
use std::path::Path;
use std::process::Output;
#[derive(Default, Debug)]
pub struct Executer {
    distri: DistriOpt,
}

impl Executer {
    pub fn new(distri: crate::config::Distri) -> Self {
        Self {
            distri: distri.into(),
        }
    }
    fn exec_cmd(&self, cmd: &str) -> Result<Output, std::io::Error> {
        std::process::Command::new("sh")
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
            let err = match result {
                Ok(_) => break Ok(()),
                Err(e) => e,
            };
            match err.kind() {
                IoErrorKind::AlreadyExists => {
                    if let Err(e) = std::fs::remove_file(&dst) {
                        break Err(e);
                    }
                }
                IoErrorKind::NotFound => {
                    if let Some(dst_parent) = dst.as_ref().parent() {
                        if let Err(e) = std::fs::create_dir_all(dst_parent) {
                            break Err(e);
                        }
                    }
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
                match self.exec_cmd(cmd) {
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

use base64;
use std::fs;
use std::io::Write;
use std::io::{self, Read};
use std::process::{Command, Output, Stdio};

use tempfile;

pub enum ExecuteResult {
    Ok(String),
    Err(String),
}
impl ExecuteResult {
    pub fn is_ok(&self) -> bool {
        if let Self::Ok(_) = self {
            true
        } else {
            false
        }
    }
    pub fn unwrap(&self) -> &String {
        match self {
            Self::Ok(msg) => msg,
            Self::Err(msg) => msg,
        }
    }
}

pub struct Executor {}

impl Executor {
    /// Execute shell code on Bash
    pub fn bash(code: &String) -> ExecuteResult {
        let output: Output = Command::new("bash")
            .arg("-c")
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .ok()
            .unwrap();
        if output.status.success() {
            ExecuteResult::Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            ExecuteResult::Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    /// Graphviz (dot) png generator
    pub fn dot(code: &String) -> ExecuteResult {
        let mut codefile = tempfile::Builder::new()
            .prefix("unidoc-")
            .suffix(".dot")
            .tempfile_in("/tmp")
            .ok()
            .unwrap();
        let mut outputfile = tempfile::Builder::new()
            .prefix("unidoc-")
            .suffix(".png")
            .tempfile_in("/tmp")
            .ok()
            .unwrap();
        write!(codefile, "{}", code).ok();
        let codefile_path = codefile.path().to_path_buf();
        let outputfile_path = outputfile.path().to_path_buf();
        let res = Self::bash(&format!(
            "dot -Tpng {} > {}",
            codefile_path.to_string_lossy(),
            outputfile_path.to_string_lossy(),
        ));
        if res.is_ok() {
            let mut buffer = Vec::new();
            outputfile.read_to_end(&mut buffer).ok();
            let encoded = base64::encode(&buffer);
            ExecuteResult::Ok(format!("data:image/png;base64,{}", encoded))
        } else {
            res
        }
    }
    /// Gnuplot svg generator
    pub fn gnuplot(code: &String) -> ExecuteResult {
        let mut codefile = tempfile::Builder::new()
            .prefix("unidoc-")
            .suffix(".gp")
            .tempfile_in("/tmp")
            .ok()
            .unwrap();
        write!(codefile, "set terminal svg;\n{}", code).ok();
        let codefile_path = codefile.path().to_path_buf();
        Self::bash(&format!("gnuplot {}", codefile_path.to_string_lossy(),))
    }
}

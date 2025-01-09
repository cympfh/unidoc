use base64::prelude::*;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use tempfile;

#[derive(Debug, PartialEq, Eq)]
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
            let encoded = BASE64_STANDARD.encode(&buffer);
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

#[cfg(test)]
mod test_executor {
    use crate::executor::*;
    #[test]
    fn test_execute() {
        assert_eq!(
            Executor::bash(&String::from("yes | head -1")),
            ExecuteResult::Ok(String::from("y\n"))
        );
        Executor::dot(&String::from("digraph { x -> y }"));
        Executor::gnuplot(&String::from("plot x"));
    }
}

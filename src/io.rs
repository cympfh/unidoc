use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read, Write};

pub fn read(input: &Option<String>) -> io::Result<String> {
    let mut content = String::new();
    if let Some(input) = input {
        let file = File::open(&input).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut content)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content)?;
    }
    if !content.ends_with('\n') {
        content += "\n"
    }
    Ok(content)
}

pub fn write(output: &Option<String>, buf: &String) -> io::Result<()> {
    if let Some(output) = &output {
        let mut file = File::create(&output)?;
        write!(file, "{}", buf)?;
    } else {
        print!("{}", buf);
    }
    Ok(())
}

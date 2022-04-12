use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read, Write};

pub fn read(input: &String) -> io::Result<String> {
    let mut content = String::new();
    if input == "-" {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content)?;
    } else {
        let file = File::open(&input).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut content)?;
    }
    if !content.ends_with('\n') {
        content += "\n"
    }
    Ok(content)
}

pub fn reads(inputs: &Vec<String>) -> io::Result<Vec<String>> {
    inputs
        .iter()
        .map(|path| read(&path))
        .collect::<Result<Vec<_>, _>>()
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

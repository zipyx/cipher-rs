use std::fs::File;
use std::io::{BufReader, BufRead, Write};

pub fn read_file_handler() -> std::io::Result<String> {
    let file = match File::open("cipher.txt") {
        Ok(file) => file,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found");
                    return Ok("File not found".to_owned());
                },
                _ => return Err(e),
            }
        }
    };

    let reader = BufReader::new(file);
    let result = reader.lines().next().unwrap();

    Ok(result.unwrap())
}

pub fn write_file_handler(data: String) -> std::io::Result<()> {
    let mut file = File::create("cipher.txt")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

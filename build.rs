use std::{process::Command, fs::File, io::Write};


fn main() -> Result<(), std::io::Error>{
    let man_data = Command::new("help2man").arg("escaper").output()?.stdout;
    let mut file = File::create("escaper-gen.1")?;
    file.write_all(&man_data)?;
    Ok(())
}

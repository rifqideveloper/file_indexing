use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn delete_line(path:&str,line:usize) -> io::Result<()>
{
    {
        let mut file_ganti = std::fs::File::create(format!("{}._pengganti",path))?;
        let mut file = BufReader::new(File::open(format!("{}",path))?);
        let mut buf = String::with_capacity(100);
        let mut i = 1;
        while file.read_line(&mut buf)? != 0 {
            if i != line {
                use std::io::Write;
                    file_ganti.write(buf.as_bytes()).expect("");
            }
            buf.clear();
            i = i + 1
        }
    }
    std::fs::rename(format!("{}._pengganti",path), format!("{}",path))?;
    Ok(())
}
pub fn read_line_index(path:&str,line:usize)-> io::Result<String>{
    use std::io::{Error, ErrorKind};
    let mut file = BufReader::new(File::open(format!("{}",path))?);
    let mut buf = String::with_capacity(100);
    let mut i = 1;
    while file.read_line(&mut buf)? != 0 
    {
        if i == line {
            return Ok(buf)
        }
        buf.clear();
        i = i + 1
    }
    Err(Error::new(ErrorKind::Other, "oh no!"))
}
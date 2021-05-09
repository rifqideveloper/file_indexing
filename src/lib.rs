use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[allow(unused_macros)]
macro_rules! delete_line_macro {
    ($path:expr,$($xline:expr),+) => {
        {
            use std::io::{BufRead};
            use std::io::Write;
            let mut file_ganti = std::fs::File::create(format!("{}._pengganti",$path)).expect("Unable to create file buffer");
            let mut file = std::io::BufReader::new(std::fs::File::open(format!("{}",$path)).expect("Unable to open"));
            let mut buf = String::with_capacity(100);
            let mut i = 1;
            while file.read_line(&mut buf).expect("Unable to read file") != 0 {
                match i {
                    $($xline =>{})*
                    _=>{file_ganti.write(buf.as_bytes()).expect("");}
                }
                buf.clear();
                i = i + 1;
            }
        }
        std::fs::rename(format!("{}._pengganti",$path), format!("{}",$path)).expect("Unable to read");
    };
    ($path:expr,$line:expr => $xline:expr )=>{
        {
            use std::io::{BufRead};
            use std::io::Write;
            let mut file_ganti = std::fs::File::create(format!("{}._pengganti",$path)).expect("Unable to create file buffer");
            let mut file = std::io::BufReader::new(std::fs::File::open(format!("{}",$path)).expect("Unable to open"));
            let mut buf = String::with_capacity(100);
            let mut i = 1;
            while file.read_line(&mut buf).expect("Unable to read file") != 0 {
                match i {
                    $line..=$xline =>{}
                    _=>{file_ganti.write(buf.as_bytes()).expect("Unable to write file");}
                }
                buf.clear();
                i = i + 1;
            }
        }
        std::fs::rename(format!("{}._pengganti",$path), format!("{}",$path)).expect("Unable to read");
    }
}
#[allow(unused_macros)]
macro_rules! read_line_index_macro {
    ($path:expr,$($xline:expr),+) => {
        {
            let mut file =  std::io::BufReader::new(std::fs::File::open(format!("{}",$path)).expect("Unable to open file"));
            let mut buf = String::new();
            let mut buf2 = String::new();
            let mut i = 1;
            use std::io::{BufRead};
            while file.read_line(&mut buf).expect("Unable to read file") != 0 
            {
                $(if i == $xline {buf2.push_str(buf.as_str())})*
                buf.clear();
                i = i + 1
            }
            buf2
        }
    };
    ($path:expr,$line:expr => $xline:expr )=>{
        {
            let mut file =  std::io::BufReader::new(std::fs::File::open(format!("{}",$path)).expect("Unable to open file"));
            let mut buf = String::new();
            let mut buf2 = String::new();
            let mut i = 1;
            use std::io::{BufRead};
            while file.read_line(&mut buf).expect("Unable to read") != 0 
            {
                match i {
                    $line..=$xline =>{buf2.push_str(buf.as_str())}
                    _=>{}
                }
                buf.clear();
                i = i + 1
            }
            buf2
        }
    };
}
#[allow(unused_macros)]
macro_rules! replase_line_macro{
    ($path:expr,$($xline:expr,$line:expr),+) => {
        {
            use std::io::Write;
            use std::io::{BufRead};
            let mut file_ganti = std::fs::File::create(format!("{}._pengganti",$path)).expect("Unable to create buffer file");
            let mut file = std::io::BufReader::new(std::fs::File::open(format!("{}",$path)).expect("Unable to open file"));
            let mut buf = String::new();
            let mut i = 1;
            while file.read_line(&mut buf).expect("") != 0 {
                match i {
                    $(n if n == $line =>{file_ganti.write($xline.as_bytes()).expect("Unable to write file");})*
                    _=>{file_ganti.write(buf.as_bytes()).expect("Unable to write file");}
                }
                buf.clear();
                i = i + 1;
            }
        }
        std::fs::rename(format!("{}._pengganti",$path), format!("{}",$path)).expect("Unable to rename ._pengganti");
    };
}
#[allow(unused_imports)]
pub(crate) use delete_line_macro;
#[allow(unused_imports)]
pub(crate) use read_line_index_macro;
#[allow(unused_imports)]
pub(crate) use replase_line_macro;
#[allow(dead_code)]
pub fn delete_line(path:&str,line:usize)->io::Result<()>{
    {
        use std::io::Write;
        let mut file_ganti = std::fs::File::create(format!("{}._pengganti",path))?;
        let mut file = std::io::BufReader::new(std::fs::File::open(format!("{}",path))?);
        let mut buf = String::with_capacity(100);
        let mut i = 1;
        while file.read_line(&mut buf)? != 0 {
            if i != line  { file_ganti.write(buf.as_bytes())?;} 
            buf.clear();
            i = i + 1;
        }
    }
    std::fs::rename(format!("{}._pengganti",path), format!("{}",path))?;
    Ok(())
}
#[allow(dead_code)]
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
    Err(Error::new(ErrorKind::Other, "line don't exist"))
}
#[allow(dead_code)]
pub fn replase_line(path:&str,replase:&str,line:usize)->io::Result<()>{
    {
        use std::io::Write;
        let mut file_ganti = std::fs::File::create(format!("{}._pengganti",path))?;
        let mut file = std::io::BufReader::new(std::fs::File::open(format!("{}",path))?);
        let mut buf = String::new();
        let mut i = 1;
        while file.read_line(&mut buf)? != 0 {
            match i {
                n if n == line =>{file_ganti.write(replase.as_bytes())?;}
                _=>{file_ganti.write(buf.as_bytes())?;}
            }
            buf.clear();
            i = i + 1;
        }
    }
    std::fs::rename(format!("{}._pengganti",path), format!("{}",path))?;
    Ok(())
}

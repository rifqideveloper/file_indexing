mod lib;

// Unix & Mac file path.
const FILE_TEST: &str = "testing/test2.txt";
// Windows file path.
// const FILE_TEST: &str = "testing\\test.txt";

fn main() {
    //? Delete specific lines in file.
    // lib::delete_line(file, 4).expect("error");
    // lib::delete_line_macro!(file,2 => 4);
    // lib::delete_line_macro!(file,2,4,6);

    //? Read line
    // if let Ok(contents) = lib::read_line_index(file, 1) {
    //     println!("{}", contents)
    // }
    // println!("{}", lib::read_line_index_macro!(file, 2));
    // lib::replase_line(file, "pengganti\n", 2).expect("error");
    // lib::replase_line_macro!(file, "pengganti\n", 2, "pengganti\n", 3);
    // lib::rotate(file, 1000).expect("msg: &str");
    // lib::pust(file, "pust\n").expect("msg: &str");
    // lib::delete_line_with_capacity(file, 1, 100).unwrap();

    // let f = lib::FileIndexing {
    //     file: FILE_TEST.to_string(),
    // };
    // println!("{}", f.readto_string().expect("error"));
    // f.replase_line("Rotation here.\n", 5).expect("error");
    // f.rotate(3).expect("error");
    // f.pust_str("new line").expect("error");
    // f.pop().expect("error");
    // println!("{}", f.len().expect("error"));
    // println!("{}", f.index(2).expect("error"));
    // f.delete().expect("error");

    println!("{}", lib::len(FILE_TEST).unwrap());
    let mut f = lib::FileIndexing_with_capacity {
        file: FILE_TEST.to_string(),
        capacity: 1000,
        buf: String::with_capacity(200),
    };

    f.rotate(2).expect("Error: in main.rs, rotate function");
}

// create test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_empty() {
        assert!(lib::len(FILE_TEST).unwrap() > 0);
    }

    #[test]
    fn file_len() {
        //file is 22 line
        assert_eq!(lib::len(FILE_TEST).unwrap(), 22);
    }
}

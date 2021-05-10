# file_indexing
## library for editing file fast and memory efficient
## crate
extern crate file_indexing;
## deleting file line
    file_indexing::delete_line(file, 4).expect("error");
## deleting file line 2 , 4 , 6
    file_indexing::delete_line_macro!(file,2,4,6);
## deleting file line 2 to 4
    file_indexing::delete_line_macro!(file,2 => 4);
## read line
    println!("{}",file_indexing::read_line_index(file, 1).expect("error"));
## read line
    println!("{}",file_indexing::read_line_index_macro!(file, 2));
## read line 2 and 5
    println!("{}",file_indexing::read_line_index_macro!(file, 2 , 5));
## read line 2 to 4
    println!("{}",file_indexing::read_line_index_macro!(file, 2 => 4));
## replase line
    file_indexing::replase_line(file, "pengganti", 2).expect("error");
## replase line 2 , 3
    file_indexing::replase_line_macro!(file,"pengganti\n",2,"pengganti\n",3);
## pop line 
    file_indexing::pop(file).expect("msg: &str");
## pust line 
    file_indexing::pust(file, "pust\n").expect("msg: &str");
## len 
    file_indexing::len(file).expect("msg: &str");
## rotate
    file_indexing::rotate(file, 1000).expect("msg: &str");
## struct FileIndexing
    let f = file_indexing::FileIndexing{file:file.to_string()};
        println!("{}", f.readto_string().expect("erro"));
        f.rotate(3).expect("erro");
        f.replase_line("replase", 4).expect("erro");
        f.pust_str("new line").expect("erro");
        f.pop().expect("erro");
        println!("{}", f.len().expect("erro"));
        println!("{}", f.index(2).expect("error"));
        f.delete().expect("error");
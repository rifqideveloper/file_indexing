# file_indexing
## library for editing file fast and memory efficient

## crate
    extern crate file_indexing;

## deleting file line
    file_indexing::delete_line(file, 4).expect("error");

## deleting file line delete_line_with_capacity
    file_indexing::delete_line_with_capacity(file, 4 , 1000).expect("error");

## deleting file line 2 , 4 , 6
    file_indexing::delete_line_macro!(file,2,4,6);

## deleting file line 2 to 4
    file_indexing::delete_line_macro!(file,2 => 4);

## read line
    println!("{}",file_indexing::read_line_index(file, 1).expect("error"));

## read line with capacity
    println!("{}",file_indexing::read_line_index_with_capacity(file, 1,1000).expect("error"));

## read line macro
    println!("{}",file_indexing::read_line_index_macro!(file, 2));

## read line 2 and 5
    println!("{}",file_indexing::read_line_index_macro!(file, 2 , 5));

## read line 2 to 4
    println!("{}",file_indexing::read_line_index_macro!(file, 2 => 4));

## replase line
    file_indexing::replase_line(file, "pengganti", 2).expect("error");

## replase line _with_capacity
    file_indexing::replase_line_with_capacity(file, "pengganti", 2 ,1000).expect("error");

## replase line 2 , 3
    file_indexing::replase_line_macro!(file,"pengganti\n",2,"pengganti\n",3);

## pop line 
    file_indexing::pop(file).expect("erro");

## pust line 
    file_indexing::pust(file, "pust\n").expect("erro");

## pust line with capacity
    file_indexing::pust_with_capacity(file, "pust\n",1000).expect("erro");

## len 
    file_indexing::len(file).expect("erro");

## rotate
    file_indexing::rotate(file, 1000).expect("erro");

## rotate with capacity
    file_indexing::rotate_with_capacity(file, 1000 ,1000).expect("erro");

## struct FileIndexing
    let f = file_indexing::FileIndexing{file:file.to_string()};
    // read all to string
        println!("{}", f.readto_string().expect("erro"));
    // rotate 3 time
        f.rotate(3).expect("erro");
    // replase line 4 to 'replase'
        f.replase_line("replase", 4).expect("erro");
    // pust new line
        f.pust_str("new line").expect("erro");
    // delete last line
        f.pop().expect("erro");
    // number of line in file
        println!("{}", f.len().expect("erro"));
    // read line 2
        println!("{}", f.index(2).expect("error"));
    // is file is empty ?
        f.is_empty();
    // delete file
        f.delete().expect("error");

## struct FileIndexing
    let f = file_indexing::FileIndexing_with_capacity{
        file:file.to_string(),
        capacity:1000,
        buf:String::with_capacity(200)
    };
    // read all to string
        println!("{}", f.readto_string().expect("erro"));
    // rotate 3 time
        f.rotate(3).expect("erro");
    // replase line 4 to 'replase'
        f.replase_line("replase", 4).expect("erro");
    // pust new line
        f.pust_str("new line").expect("erro");
    // delete last line
        f.pop().expect("erro");
    // number of line in file
        println!("{}", f.len().expect("erro"));
    // read line 2
        println!("{}", f.index(2).expect("error"));
    // is file is empty ?
        f.is_empty();
    // delete file
        f.delete().expect("error");
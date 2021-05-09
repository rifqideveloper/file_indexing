# file_indexing
extern crate file_indexing;
fn main(){
    let file = "file.txt";
    //deleting file line
    file_indexing::delete_line(file, 4).expect("error");

    //deleting file line 2 , 4 , 6
    file_indexing::delete_line_macro!(file,2,4,6);

    //deleting file line 2 to 4
    file_indexing::delete_line_macro!(file,2 => 4);

    //read line
    println!("{}",file_indexing::read_line_index(file, 1).expect("error"));

    //read line
    println!("{}",file_indexing::read_line_index_macro!(file, 2));

    //read line 2 and 5
    println!("{}",file_indexing::read_line_index_macro!(file, 2 , 5));

    //read line 2 to 4
    println!("{}",file_indexing::read_line_index_macro!(file, 2 => 4));

    //replase line
    file_indexing::replase_line(file, "pengganti", 2).expect("error");
    
    //replase line 2 , 3
    file_indexing::replase_line_macro!(file,"pengganti\n",2,"pengganti\n",3);

}

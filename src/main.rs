mod lib;
fn main(){
    //file
    let file = "testing\\file.txt";
    //deleting file line
    //lib::delete_line(file, 4).expect("erro");
    //lib::delete_line_macro!(file,2 => 4);
    //lib::delete_line_macro!(file,2,4,6);
    //read line
    //match lib::read_line_index(file, 1){
     //   Ok(o)=>{println!("{}",o)}
      //  Err(_)=>{}
    //}
    //println!("{}",lib::read_line_index_macro!(file, 2));
    //lib::replase_line(file, "pengganti", 2).expect("error");
    lib::replase_line_macro!(file,"pengganti\n",2,"pengganti\n",3);
    
}
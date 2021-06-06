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
    //lib::replase_line_macro!(file,"pengganti\n",2,"pengganti\n",3);
    //lib::rotate(file, 1000).expect("msg: &str");
    //lib::pust(file, "pust\n").expect("msg: &str")
    //lib::delete_line_with_capacity(file, 0, 100);
    /*
    let f = lib::FileIndexing{file:file.to_string()};
      println!("{}", f.readto_string().expect("erro"));
      f.rotate(3).expect("erro");
      f.replase_line("replase", 4).expect("erro");
      f.pust_str("new line").expect("erro");
      f.pop().expect("erro");
      println!("{}", f.len().expect("erro"));
      println!("{}", f.index(2).expect("error"));
      f.delete().expect("error");
      */
      println!("{}",lib::len(file).unwrap());
      let mut f = lib::FileIndexing_with_capacity{
        file:file.to_string(),
        capacity:1000,
        buf:String::with_capacity(200),
      };

      f.rotate(2);
}
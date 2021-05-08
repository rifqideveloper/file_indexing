
mod lib;
fn main(){
    //file
    let file = "testing\\file.txt";
    //deleting file line
    lib::delete_line(file, 3).expect("Erro");
    //read line
    match lib::read_line_index(file, 1){
        Ok(o)=>{println!("{}",o)}
        Err(_)=>{}
    }
}
use std::io;
fn main(){
    let mut y =String::new();
     println!("Enter the word you want to find length:");

     io::stdin()
     .read_line(&mut y)
     .expect("error ");
    let z=y.trim();
let length=get_string_len(z);
println!("output is {}",length);
}
fn get_string_len(str: &str)-> usize{
    str.chars().count()
}

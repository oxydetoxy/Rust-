use std::io;
fn main(){
    let mut  z =String::new();
    println!("Enter the number you want to find:");
    io::stdin()
    .read_line(&mut z)
    .expect("might be some error");
 let num :i32 = z.trim().parse().expect("enter valid number");
    
    print!("Fibbonacci number is: {}\n",fib(num));
}
fn fib(num: i32)->i32{
    let mut  x =0;
    let mut y=1;
     if num==0{
     return x;}
    
     else if num==1{
     return y;}
     for _i in 0..(num-1){
     let _temp=y;
     y=y+x;
     x=_temp;
     }
     return y;
}

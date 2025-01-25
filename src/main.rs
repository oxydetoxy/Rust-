use std::collections::HashMap;
fn is_even(v:&mut  Vec<i32>){
    let mut i=0;
    while i<v.len(){
        if v[i]%2!=0{
            v.remove(i);

        }
        else {
            i+=1;
        }
    }
}

fn main(){
     let mut r = Vec::new();
     r.push(4);
     r.push(34);
     r.push(1);
  r.push(7);
  is_even(&mut r);
 println!("{:?}",r);



}
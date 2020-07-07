
// fn main() { 
//     let arry = [1,2,4,6,8,9,7,5];
//      let mut element = 5;
//      loop {
//          element -= 1;
//       if element < 1 {
//          break  };
//      println!("the even number is [{}]",arry[element]);
//        };
//        let index2 = 4;
//        let index = 5;
//        let x = arry[index];
//        let y = arry[index2];
//        println!("the larges valu is {}and",x);
//        println!("als {}",y);
//     let number= add();
//     println!("{}",number);
// }   
// fn add()->u32 {
    
//        let arry = [1,2,4,6,8,9,7,5];
//        let index2 = 4;
//        let index = 5;
//        let x = arry[index];
//        let y = arry[index2];
//        let add = x+y;
//        //println!("the sum of large number is {}", add);
//        add
//     }
//std::result::Result
use std::io;
fn main(){
  let mut sting_type = String::new();
  io::stdin().read_line(& mut sting_type).expect( "failed to read line");
         // println!("{}",sting_type);
   let  number :usize =sting_type.trim().parse().expect("failed to convert") ;   
    let m = number +1;   
   println!("number is that {}",m);
 
 
 
 
  }



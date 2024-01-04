fn main() {
    let result = add_number(5.0,3.0);
    println!("The result is {}", result);
}

fn add_number(a: f32, b: f32) -> f32 {
   let result = a/b;

   if result > 10.0{
     return result;
   }
   result - 2.0
}

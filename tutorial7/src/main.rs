fn main() {
    // operators && || !
    let cond1  = (2 as f32) <= 4.6;
    let cond2 = cond1 && true;

    // Conditional Programming
    if !cond2 {
        println!("The condition is {}", cond2);
    }
    else{ //else if
        println!("Not so true");
    }
    
}

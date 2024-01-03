// Variable, Constant and Shadowing
fn main() {
    let x = 4;
    println!("x is {}", x);

    let mut y = 4;
    println!("y is {}", y);
    y += 1;
    println!("y is {}", y);

    let z = &y;
    println!("z is {}", z);
    
    {
        let x = 9;
        println!("x is {}", x);
    }

    println!("x is {}", x);


    const SECONDS_IN_MINUTE: u32 = 60;

    println!("Seconds in minutes {}", SECONDS_IN_MINUTE)
}

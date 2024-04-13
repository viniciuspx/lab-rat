// this simple example is to show the use of mutable variables
fn main() {
    // without the keyword mut the variable cant get assigned again
    // implcit typing in this case i32
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // constants cant change value, and must have the type defined in this case u32
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // more shadowing
    let y = 5;
    let y = y + 1;
    // different from assigning we are using let again to re-define the varible y
    // in this way we "shadow" the previous value
    {
        // this variable only exists in this scope, once we are out >>puff
        let y = y * 2;
        println!("The value of x in the inner scope is {y}");
    }

    println!("The value of x is: {y}");
}

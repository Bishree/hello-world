fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("the value of x is: {}", x);
    let y = {
        let x = 3;
        x+3
    };
    println!("The value of y is: {}", y);
}

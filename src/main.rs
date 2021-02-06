use std::io;
use std::iter::Enumerate;

fn main() {
    /// TODO
    /// read the input Fahrenheit & celsius from input
    /// for celsius press C and for Fahrenheit press F
    loop {
        println!("for celsius press C and for Fahrenheit press F");
        let mut degree_type = String::new();

        io::stdin().read_line(&mut degree_type).expect("something wroooong!");
        // println!("degree type is: {}", degree_type);
        // let = match degree_type.trim().parse() {
        // //     Ok(num)=> num,
        // //     Err(_) => continue,
        // // };
        let mut int_degree_type = degree_type.trim().parse().unwrap();
        f_conv(int_degree_type);
        println!("The celsius value is: {} ", degree_type);
        }
}


    // create a function for the convert formula
fn c_conv (f:u32){

        let c = (f-32) * (5 / 9);
        println!("The value of the Celsius is: {}", c);
    }

fn f_conv (c:u32){
        let mut f = (c * (9 / 5)) + 32;
        println!("The value of the Fahrenheit is: {}", f);
    }

    // call the function and print result
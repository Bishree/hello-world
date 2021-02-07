use std::io;
use std::iter::Enumerate;
use std::convert::AsRef;

fn main() {
    /// TODO
    /// read the input Fahrenheit & celsius from input

    loop {
        println!("for celsius press C and for Fahrenheit press F");
        let mut degree_type = String::new();
        io::stdin()
            .read_line(&mut degree_type).unwrap();
        /// defreciate the input for the required tempreture degree type
        match degree_type.as_str().trim() {
            "ahmed" => {
                println!("Enter the Celsius degree plz:/n");
                let mut degree_int = String::new();
                io::stdin().read_line(&mut degree_int).expect("you need to enter an integer");
                let degree_int_number =degree_int.trim().parse().unwrap();
                f_conv(degree_int_number);
                }
            "ahmed2" => {
                println!("Enter the Fahrenheit degree plz:/n");
                let mut degree_int = String::new();
                io::stdin().read_line(&mut degree_int).expect("you need to enter an integer");
                let degree_int_number =degree_int.trim().parse().unwrap();
                c_conv(degree_int_number);
                }

            _ => {
                println!("this is the end of the match")
            }
        }println!("Nothing found");
        // f_conv(int_degree_type);
        // println!("The celsius value is: {} ", degree_type);
    }
        // let mut int_degree_type = degree_type.trim().parse().unwrap();

}


    // create a function for the convert formula
fn c_conv (degree:i32){
        let mut c = (degree - 32) * (5/9);
        println!("The value of the Celsius is: {}", c);
    }

fn f_conv (degree:i32){
        let mut f = (degree * (9 / 5)) + 32;
        println!("The value of the Fahrenheit is: {}", f);
    }

    // call the function and print result
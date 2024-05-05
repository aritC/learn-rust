use std::io;
use std::process;

fn fibo(n:u32)-> u32{

    if n==0 || n == 1{
        1
    }else{
        n * fibo(n-1)
    }
}


fn main() {
    println!("Enter the which fibonacci number you want to find: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");
    
    let n: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_)=>{
            println!("Not a valid number!");
            process::exit(1);
        }
    };

    let fibo = fibo(n);
    println!("Fibonacci in the {number}th position is: {fibo}");
}

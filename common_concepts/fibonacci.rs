use std::io;

fn nth_fibonacci(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        x => nth_fibonacci(x-1) + nth_fibonacci(x-2)
    }
}

fn main () {
    
    println!("Which Fibonacci number did you want?");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line.");
    let index: u32 = index.trim().parse()
	.expect("Please input a number.");
    
    println!("{}", nth_fibonacci(index));

}

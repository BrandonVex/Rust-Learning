// rust

fn main() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
} 

//------------------------------------------------------------

fn main() {
    let n: i32 = 5;

    let big_n: i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32
        };

    println!("{} -> {}", n, big_n);
} 

//------------------------------------------------------------




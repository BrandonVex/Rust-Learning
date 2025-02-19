// Loop is usually used together with break or continue.

fn main() {
    let mut count: u32 = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

//------------------------------------------------------------

fn main() {
    let mut counter: i32 = 0; // counter variable

    let result: i32 = loop {
        counter += 1; // increment counter

        if counter == 10 { 
            break counter * 2; // break with a value
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

//------------------------------------------------------------

// It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.
fn main() {
    let mut count: i32 = 0;

    // nested loops
    // `outer refers to the outer loop and `inner 1/`inner 2 refer to the inner loops
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!()
    // unimplemented!()
    // todo!()
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_


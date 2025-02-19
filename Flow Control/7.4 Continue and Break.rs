fn main() {
    let mut n = 0;
    for i in 0..=100 { // 100 is included because of the "="
       if n == 66 {
           break // break out of the loop
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
//------------------------------------------------------------

fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       
       break
    }

    assert_eq!(n, 66);

    println!("Success!");
}
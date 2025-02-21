fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // you use the matches! macro to check if a value matches a specific pattern
    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9')) // given argument == true
    }

    println!("Success!");
} 

// as you iterate through the array, the matches! macro checks if the value of the current element is a letter or a digit.
// If it is, the assert! macro will return true, and the loop will continue to the next element.
// If the value is not a letter or a digit, the assert! macro will return false, and the program will panic.

//-------------------------------------------------------------------------------------------------

enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;
    let v: Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo]; // dynamic sizing
    
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}
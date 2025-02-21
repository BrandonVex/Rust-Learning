// For some cases, when matching enums, match is too heavy. We can use if let instead.

fn main() {
    let o: Option<i32> = Some(7);

    if let Some(i) = o { // inner value of o is assigned to i = 7
        println!("This is a really long string and `{:?}`", i);

        println!("Success!");
    }
}

// if let is a shorthand for a match that only matches one case.
// In this case, if the value of o is Some(i), then the code block will execute.

//-------------------------------------------------------------------------------------------------


// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a: Foo = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}

// The if let statement is used to match the value of a variable against a pattern.
// In this case, the pattern is Foo::Bar(i), which matches the value of a if it is Foo::Bar.

//-------------------------------------------------------------------------------------------------

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a: Foo = Foo::Qux(10);

    match a {
        Foo::Bar => println!("a is a Bar"),
        Foo::Baz => println!("a is a Baz"),
        Foo::Qux(i) => println!("a is a Qux with value {}", i)
    }
}
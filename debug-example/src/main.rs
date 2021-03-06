#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };


    // normal print (needs to implement the Display trait)
    // println!("{}", peter);
    // Debug print
    println!("{:?}", peter);
    // Pretty print
    println!("{:#?}", peter);
}



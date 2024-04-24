fn increment(x : Option<i32>) -> Option<i32>{

    match x {
        Some(x) => Some(x + 1),
        None => None
    }

}

fn add_optionals(x : Option<i32>, y : Option<i32>) -> Option<i32> {

    return Some(x? + y?)

}

fn main(){

    let some_number = Some(5);
    let some_text = Some(String::from("Hello, world!"));
    let no_value : Option<i32> = None;

    //Wyłuskanie wartości za pomocą match
    let lol = increment(some_number);
    println!("{:?}", lol);

    let unwraptest1 : Option<i32> = Some(55);
    let unwraptest2 : Option<i32> = None;
    let lol1 : Option<String> = Some(String::from("Hello, world!"));
    println!("{}", unwraptest1.unwrap());
    println!("{}", lol1.expect("Aha lol")); //Tu dla nona i tak spanikuje
    println!("{}", unwraptest2.unwrap_or(0));
    println!("{}", unwraptest2.unwrap_or_default()); //typ musi realizować trait Default

    println!("{:?}", add_optionals(Some(5), Some(10)));
    

}
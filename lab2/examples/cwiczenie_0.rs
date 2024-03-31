fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!
    let mut start = 0;
    let mut end = input.len();

    for (i, byte) in input.bytes().enumerate(){

        if byte as char != ' '{
            start = i;
            break;
        }

    }

    for (i, byte) in input.bytes().enumerate().rev(){ //enumerate musi być przed rev, żeby się indeksy zaczynały od końca

        if byte as char != ' '{
            end = i + 1;
            break;
        }

    }

    input[start..end].to_string()

}

fn compose_me(input: &str) -> String {
    // Adds " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
    
}

fn replace_me(input: &str) -> String {
    // Replaces "cars" in the string with "balloons"!
    input.to_string().replace("cars", "balloons")
}
 
fn main() {
    assert_eq!(trim_me("Hello!     "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");
 
    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");
 
    assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
    assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
}
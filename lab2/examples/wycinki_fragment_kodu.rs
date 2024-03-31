fn main() {
    let mut s = String::from("text that illustrates slices");
    let first = first_word(&s[..]); // <- immutable borrow
    
    //s.clear(); //tries to modify text //<- mutable borrow
 
    println!("{first}");
}   
 
fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    s
}
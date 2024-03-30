fn main(){

    //typ string
    let mut s = String::from("Hello");
    s.push_str(" world");
    println!("{}", s);

    /*Główne zasady określania własności wartości:

    Każda wartość w Rust ma zmienną, która jest nazywana jej właścicielem (owner).
    Właściciel może być tylko jeden w danym momencie.
    Gdy właściciel wyjdzie poza zakres, wartość zostanie usunięta. */

}
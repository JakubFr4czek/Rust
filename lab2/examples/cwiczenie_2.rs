// Napisz funkcję, która dla danego adresu URL zwraca krotkę zawierającą:
// - domenę,
// - ścieżkę żądania,
// - parametry zapytania

fn decomposeURL(input : &str){

    let domainStart = input.find("://") as u32;
    let domainEnd = input.find("/") as u32;

    println!("{}, {}", domainStart, domainEnd);

}

fn main(){

    let url = String::from("https://www.example.com/path/to/resource?param1=value1&param2=value2");
    decomposeURL(&url);

}
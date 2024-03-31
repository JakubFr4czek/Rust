// Napisz funkcję, która dla danego adresu URL zwraca krotkę zawierającą:
// - domenę,
// - ścieżkę żądania,
// - parametry zapytania

fn decomposeURL(input : &str) -> (String, String, String){

    let domain_start = input.find("://").unwrap();
    let domain_end = domain_start + 3 + input[domain_start + 3..].find("/").unwrap();

    let path_start = domain_end;
    let path_end = path_start + input[path_start..].find("?").unwrap();

    // println!("{}, {}", domain_start, domain_end);
    return (input[domain_start + 3..domain_end].to_string(),
            input[path_start..path_end].to_string(),
            input[path_end + 1..].to_string());
}

fn main(){

    let url = String::from("https://www.example.com/path/to/resource?param1=value1&param2=value2");
    let (_url, _path, _arguments) = decomposeURL(&url);

    println!("url = {_url}, path = {_path}, arguments = {_arguments}");

}
// 1. Generator haseł: Napisz funkcję, która wygeneruje losowy ciąg znaków o określonej długości.
// Wykorzystaj w tym celu funkcję generowania liczb losowych, której przykład użycia znajdziesz na poniższym fragmencie kodu
//
// use rand::{thread_rng, Rng};
// 
// fn main() {
//     let mut rng = rand::thread_rng();
//     let rand_number = rng.gen_range(0..10);
//  
// }

use rand::{thread_rng, Rng};

fn random_password(length : u32) -> String{

    let mut rng = rand::thread_rng();

    let mut s : String = "".to_string();

    //zakres znaków [33, 94]
    for _ in 0..length{

        let rand_number = rng.gen_range(33..94 + 1) as u8;
        s.push(rand_number as char);

    }

    return s;

}

fn main(){

    let s1 = random_password(5);
    println!("s1 = {s1}");

    let s2 = random_password(35);
    println!("s2 = {s2}");

}
fn main(){

    //Pożyczka zmiennej
    let s1 = String::from("Ala ma kota");
    let s2 = resell_string(&s1);
    println!("s1 = {}, s2 = {}", s1, s2);

    //Mutowalna pożyczka zmiennej
    let mut s1 = String::from("Ala ma kota");
    let s2 = resell_string_mutable_reference(&mut s1);

    /*
    Reguły dotyczące referencji

    W dowolnym momencie programu, dla jednej wartości możemy mieć tylko jeden z poniższych przypadków:

        jedną mutowalną referencję,
        dowolną liczbę niemutowalnych referencji.

    Ponadto wszystkie odwołania muszą być ważne, ie. muszą odnosić się do wartości, które nie są poza zakresem.
    */

}

fn resell_string(napis : &String) -> String{
    napis.to_uppercase()
}

fn resell_string_mutable_reference(napis : &mut String){

    napis.truncate(1);

}
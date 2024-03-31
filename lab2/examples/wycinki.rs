fn main(){

    //Wycinek - odwołanie do fragmentu tekstu bez pożyczenia wartości <- nie prawda
    let s = String::from("Ala ma kota");
    let slice = &s[0..3]; //trzeba pamiętać o tym &
    println!("{:?}", slice);

    //Tylko górne, dolne i bez ograniczeń
    let s = String::from("Ala ma kota");
    let slice1 = &s[4..];
    let slice2 = &s[..3];
    let slice3 = &s[..];
    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);

    //UWAGA! - wycinki należy traktować jako niemutowalne wypożyczenie wartości

}
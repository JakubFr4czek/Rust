fn main(){

    //Przenoszenie własności
    let mut s1 = String::from("Ala ma kota");
    let s2 = s1; //s1 sprzedał swojego stringa do s2
    println!("s2 = {}", s2);
    //println!("s1 = {}", s1); //error, s1 sprzedał stringa i już go nie ma

    //Płytka kopia
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); //x i y są na stosie i wskazują na tą samą wartość

    //Deep copy
    s1 = s2.clone();
    println!("s2 = {}, s1 = {}", s2, s1);

    //OWnership, a funkcje
    let number : i32 = 5;
    let napis = String::from("Ala nie ma kota");
    printNumber(number); //shallow copy
    printNumber(number);
    printString(napis); //napis sprzedany
    //printString(napis); //error

    //Ownership zwracany przez funkcje
    let mut napis = String::from("Ala ma psa");
    napis = resellString(napis);
    printString(napis);

}

fn printNumber(number: i32){
    println!("{}", number);
}

fn printString(napis: String){
    println!("{}", napis);
}

fn resellString(napis: String) -> String{
    napis
}
fn main(){

    //zmienne niemutowalne
    let number : i32 = 1;
    println!("{} to liczba całkowita 32 bitowa", number);

    //inferencja typu
    let number = 5;
    println!("{} - kompilator domyślił się, że to 32 bitowy int", number);

    //Poprawne nadpisywanie niemutowalnej zmiennej
    let spaces = "    "; // &str
    let spaces = spaces.len(); // spaces variable now points to len of text and its type is usize
    println!("Size of spaces is {}", spaces);

    //Niepoprawne nadpisywanie niemutowalnej zmiennej
    let number : u32 = 12;
    //number = 6; //this wont compile
    println!("{}", number);

    //Zmiana zmiennej w scopie
    let x = 5; 
    let x = x + 1; 
    { 
        let x = x * 2; 
        println!("The value of x in the inner scope is: {}", x); 
    } 
    println!("The value of x is: {}", x); 

    //Zmienne mutowalne
    let mut _number : u32 = 12;
    _number = 6;
    println!("{}", number);

    //Stałe
    const DB_PORT : u32 = 5432;

}
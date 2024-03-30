fn main(){

    //Typy skalarne
    let _x = 5 + 5;

    //Inferencja nie działa przy parsowaniu wartości
    let _x : u32 = "432".parse().expect("Wrong number!");

    //Visual separator
    let _x = 100_000; //W pythonie jest to samo

    //Liczba zapisana szesnastkowo
    let _x = 0xff;

    //Liczba zapisana ósemkowo
    let _x = 0o77;

    //Liczba zapisana binarnie
    let _x = 0b111_000;

    //Nadanie typu za pomocą przyrostka
    let _x = 15i32;

    //Liczba zmiennoprzecinkowa
    let _x = 2.0;

    //Jawne rzutowanie typów
    let _x = 3i32 + 4u32 as i32;

    //Zmienna boole'owska
    let _flag : bool = true;
    let _flag = true; //To wystarcza

    //Typ znakowy
    let _char : char = 'a';
    let _char = 'a'; //To wystarcza

}
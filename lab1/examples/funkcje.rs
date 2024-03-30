fn main(){

    //Nazwy zmiennych i funkcji są pisane snake_casem
    let result = foo(13);
    println!("Funkcja zwróciła: {}", result);

    let result2 = foo2();
    println!("Foo2: {}", result2);
}

fn foo(x : i32) -> bool{
    if x == 13 {
        return true;
    }
    return false;
}

fn foo2() -> i32{
    12 //funkcja zwraca ostatnia wartosc
}
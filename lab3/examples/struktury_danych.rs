struct EmptyStructure;

struct Tuple(i32, i32, i32);

struct Point{
    x : i32,
    y : i32,
}

#[derive(Debug)] //Żeby można było wypisać stukturę przy użyciu {:?}
struct Rectangle{
    a : f32,
    b : f32,
}

fn main(){

    //pusta strutura
    let es = EmptyStructure;

    //krotka
    let tuple = Tuple(1, 2, 3);

    //Zwykla struktura
    let eg = Point{x : 2, y : 3};

    //Wypisanie i dostęp do elementów struktury
    let rect = Rectangle{a : 5.0, b : 10.0};
    println!("{:?}", rect);
    println!("a = {}, b = {}", rect.a, rect.b);

    //Instancja na bazie innej instancji
    let rect2 = Rectangle{a : 6.0, ..rect};
    println!("{:?}", rect2);

}
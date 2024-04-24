struct Point{
    x : i32,
    y : i32
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main(){

    //Zwykle dopasowanie wartosci
    let x:i32 = 64;

    match x{

        1 => println!("Jeden"),
        2 => println!("Two"),
        3 => println!("Drei"),
        4 => println!("Quattro"),
        _ => println!("(o _ o)")

    }

    //Wzorce wielokrotne

    match x{

        1 | 2 => println!("Jeden lub dwa"),
        2 | 3 | 4 | 64 => println!("Dwa lub trzy lub cztery lub sześćdziesiąt pięc"),
        _ => println!("Jakies cos nwm")
    }

    //Dopasowanie zakresów

    match x{

        1..=5 => println!("Pomiędzy jeden i pięć"), //range is incusive apparently
        6..=77 => println!("Pomiędzy sześć i siedemdziesiąt siedem"),
        _ => println!("Defaultowy zakres")

    }

    //Dekonstrukcja struktury
    let p = Point{x : 5, y : 123};

    match p{

        Point{x : 5, y} => println!("Y jest dowolny, a x to pięć!!!!!, a oto dowód tu -> y : {}", y),
        Point{x, y : 214} => println!("X jest dowolny, a y to dwieście czternaście, a oto dowód o tutaj patrz -> x: {}", x),
        Point{x, y} => println!("A tutaj jest ten, x i y dowolne, ale to sie wykona tylko jak powyższe wzorce się nie dopasują bo match nie jest fall-trough czy coś w tym stylu")

    }

    //Pominięcie atrybutów

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin{

        Point3D{x, ..} => println!("Zakładamy klapki na oczy i widzimy tylko x'a: {}", x),
        _ => ()
        

    }

    //Instrukcja warunkowa w dopasowaniu wzorca

    let cos = Some(777);

    match cos{

        Some(x) if x % 2 == 0 => println!("Ta liczba jest najzwyczajniej w świecie parzysta"),
        Some(x) if x % 2 == 1 => println!("Ta liczba jest nieparzysta"),
        _ => ()

    }

    //Kombinacja strażnika z dopasowaniem wzorca
    let liczba:i32 = 404;
    let bol:bool = false;

    match liczba{

        404 | 403 | 402 if bol == false => println!("Grubo"),
        _ => println!("Cienko")

    }

    //if let

    if let Some(777) = cos {
        println!("Ah yes, equality");
    }

}
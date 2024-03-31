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

impl Rectangle{//Miejsce implementacji metod dla struktury Rectangle

    fn area(&self) -> f32{ //trzeba przekazać self, tak jak w pythonie

        self.a * self.b

    }

    fn scale(&mut self, scalar : f32){ //&mut self, żeby można było zmienić warości a i b

        self.a = self.a * scalar;
        self.b = self.b * scalar;

    }

    fn new_square(a : f32) -> Rectangle{
        Rectangle{a, b : a}
    }

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
    //rect2.a = 3.0; //error, struktura niemutowalna

    //Mutowalne struktury
    let mut rect2 = Rectangle{a : 3.0, ..rect};
    rect2.a = 4.0;
    println!("{:?}", rect2);

    //Wywołanie metody ze struktury
    println!("{}", rect2.area());

    //Mnożenie prostokąta przez skalar
    rect2.scale(3.0);
    println!{"{:?}", rect2};

    //Tworzę kwadrat
    let sq = Rectangle::new_square(2.0);
    println!("{:?}", sq);

}
#[derive(Debug)] //Żeby można było wypisać stukturę przy użyciu {:?}
struct Rectangle{
    a : f32,
    b : f32,
}

impl Rectangle{//Miejsce implementacji metod dla struktury Rectangle

    /*fn area(&self) -> f32{ //trzeba przekazać self, tak jak w pythonie
        println!("Rectangle area");
        self.a * self.b

    }*/

    fn scale(&mut self, scalar : f32){ //&mut self, żeby można było zmienić warości a i b

        self.a = self.a * scalar;
        self.b = self.b * scalar;

    }

    fn new_square(a : f32) -> Rectangle{
        Rectangle{a, b : a}
    }

}

trait Shape{ //Trochę taki Javowy interfejs

    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;

    fn describe(&self){
        println!("Shape");
    }

}

impl Shape for Rectangle{

    fn area(&self) -> f32{
        println!("Shape area");
        self.a * self.b

    }

    fn perimeter(&self) -> f32{
        2.0 * self.a + 2.0 * self.b
    }

    /*fn describe(&self){
        println!("Rectangle");
    }*/

}


fn main(){

    //Najpierw wywołuje się metoda zaimplementowana w structcie, a potem dobiero w traicie
    let r = Rectangle{a : 3.0, b : 2.5};
    println!("{}", r.area());
    r.describe();

}
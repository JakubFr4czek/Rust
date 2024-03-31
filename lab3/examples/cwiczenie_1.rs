use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)] //so add wont consume vectors
struct Vec2D{

    x : f32,
    y : f32,

}

impl Vec2D{

    fn show(&self){

        println!("x = {}, y = {}", self.x, self.y);

    }

    fn to_unit_vector(&mut self){

        let temp = f32::powf(self.x.powf(2.0) + self.y.powf(2.0) , 0.5);

        self.x = self.x / temp;
        self.y = self.y / temp;

    }

    fn equal(&self, v : &Vec2D) -> bool{

        if self.x == v.x && self.y == v.y {
            return true;
        }
        return false;

    }

}

impl Add for Vec2D{

    type Output = Self;

    fn add(self, other : Vec2D) -> Self{

        return Vec2D{x : self.x + other.x, y : self.y + other.y};

    }

}

impl Sub for Vec2D{

    type Output = Self;

    fn sub(self, other : Vec2D) -> Self{

        return Vec2D{x : self.x - other.x, y : self.y - other.y};

    }

}

fn main(){

    let mut v = Vec2D{x : 1.0, y : 4.0};
    v.to_unit_vector();
    v.show();

    let u = Vec2D{..v};

    println!("{}", u.equal(&v));

    println!("u + v = {:?}, u - v = {:?}", u.add(v), u.sub(v));
}
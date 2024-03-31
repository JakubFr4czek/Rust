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

    fn length(&self) -> f32{
        return f32::powf(self.x.powf(2.0) + self.y.powf(2.0), 0.5);
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

struct Ship{

    pos : Vec2D,
    dir : f32,

}

impl Ship{

    fn rotate(&mut self, degrees : f32){
        self.dir += degrees;
    }

    fn translate(&mut self, vec : Vec2D){
        self.pos = self.pos.add(vec);
    }

    fn show_pos(&self){
        self.pos.show();
    }

    fn dist_from_point(&self, point : Vec2D) -> f32{
        return point.sub(self.pos).length();
    }

    fn dist_from_ship(&self, ship : Ship) -> f32{
        return ship.pos.sub(self.pos).length();
    }

}

fn main(){




}
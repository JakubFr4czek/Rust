struct Pair<T>{
    x : T,
    y : T,
}

struct Pair2<T, K>{
    x : T,
    y : K,
}


fn extract_x<T>(p : Pair<T>) -> T {
    p.x
}

impl <T> Pair<T> {
    fn extract_x(self) -> T {
        self.x
    }
}

impl<T: PartialOrd + Copy> Pair<T> {
    fn bigger(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}


fn main(){

    let pi = Pair{x : 5, y : 3};
    let pf = Pair{x : 15f64, y : 12f64};

    let pw = Pair2{x : 5i32, y : 6f64};

    println!("{:?}", pi.bigger());

    //println!("{:?}", extract_x(pi));
    //println!("{:?}", pf.extract_x());
}
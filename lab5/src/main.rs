struct Pair<T : Copy>{

    x : T,
    y : T,

}

impl <T : Copy>  Pair<T : Copy> {

    fn to_tuple Pair<T>(&self) -> (T, T){
        (self.x, self.y);

    }

}
fn main() {
    
    let p = Pair{x: 1, y: 2};

    let t = p.to_tuple();

    println!("{:?}", t);

}

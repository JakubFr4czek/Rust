fn main(){

    //Kod bloku
    let x = {

        if 1 == 1{
            14
        }else{
            15
        }

    };
    println!("x = {}", x);

    //pętla loop
    let mut i = 0;
    let y = loop{ //loop zwraca wartość

        if i > 10{
            break i; //break zwraca tą wartość XD?
        }

        i += 1;
    };
    println!("y = {}", y);

    //pętla while
    let mut i = 0;
    while i < 10{

        i += 1;

    }
    println!("i = {}", i);

    //ranged for
    let arr = [1, 2, 3, 4, 5, 4, 3, 2, 1];
    for x in arr{
        println!("{}", x);
    }

    //for loop
    let arr = [1, 2, 3, 4, 5, 4, 3, 2, 1];
    for i in 0..arr.len(){
        println!("{}", arr[i]);
    }

}
fn max<T : IntoIterator> (arr : T){

    let mut max = arr[0];

    for el in arr{

        if el > max{
            max = el;
        }        

    }

    max

}

fn main(){

    let arr = [1, 2, 3, 4, 5];

    println!("{:?}", max(arr));

}
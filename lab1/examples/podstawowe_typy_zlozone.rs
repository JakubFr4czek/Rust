fn main(){

    //Krotka może przechowywać różne typy danych
    let tuple = (3i32, 'x', "ok", 2.0);
    println!("Tuple contents {:#?}", tuple); //Można tak wypisać tylko do 12 elementó
    println!("Pierwsza wartość w krotce to: {}", tuple.0);

    //Dekonstrukcja krotki (jak w pythonie)
    let coordinates = (-1.7, 6.0, 13.4);
    let (x, y, z) = coordinates;
    println!("x = {}, y = {}, z = {}", x, y, z);

    //Krotka jednoelementowa
    let tuple = (5i32, );
    println!("Jednoelementowa krotka {:#?}", tuple);

    //Tablice
    let primes = [2, 3, 5, 7, 11]; //typ tablicy to [i32, 5]
    
    //Tablica wypełniona takimi samymi elementami
    let zeros = [0; 100]; //srednik w środku ma znaczenie

    //Tablicy są indeksowane
    println!("Indeks zerowy tablicy: {}", primes[0]);

    //Długość tablicy
    println!("Długość tablicy to: {}", primes.len());

    //Tablice wielowymiarowe
    let multisimensional = [[1, 2], [3, 4], [5, 6]];
}
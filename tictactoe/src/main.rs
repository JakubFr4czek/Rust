use std::io;

fn place(map : &mut [[char; 3]; 3], x : usize, y : usize, sign : char) -> bool{
    if x < 3 && y < 3{
        
        if map[x][y] == ' ' && (sign == 'X' || sign == 'O'){
            
            map[x][y] = sign;
            return true;
        }

    }

    return false;

}

fn init_game() -> ([[char; 3]; 3], [char; 2]) {

    let map = [ [' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' '] ];
    
    let mut user_input = String::new();
 
    println!("Welcome to tictactoe! Choose configuration (X, O) or (O, X) [1/2]: ");

    let _ = io::stdin().read_line(&mut user_input);

    if user_input.trim() == "1" {
        return (map, ['X', 'O'])
    }else{
        return (map, ['O', 'X'])
    }

}

fn print_map(map : [[char; 3]; 3]){

    for (i, row) in map.iter().enumerate() {
        println!(" {} | {} | {} ", row[0], row[1], row[2]);
        if i < 2 {
            println!("-----------");
        }
    }

}

fn check_for_winner(map : [[char; 3]; 3]) -> bool{

    for i in 0..3{

        if map[i][0] != ' ' && map[i][0] == map[i][1] && map[i][1] == map[i][2]{
            return true;
        }
        if map[0][i] != ' ' && map[0][i] == map[1][i] && map[1][i] == map[2][i]{
            return true
        }
    }

    if map[0][0] != ' ' && map[0][0] == map[1][1] && map[1][1] == map[2][2]{
        return true;
    }

    if map[0][2] != ' ' && map[0][2] == map[1][1] && map[1][1] == map[2][0]{
        return true;
    }

    return false;

}

fn game(){

    let (mut map, configuration) = init_game();
    
    let mut whos_turn = 0usize;

    let mut placed = 0;

    print_map(map);

    loop {
            
        let mut x_coord = String::new();
        let mut y_coord = String::new();

        if whos_turn == 0{
            println!("Player1, provide x coordinate of your move [0/1/2]: ");
        }else{
            println!("Player2, provide x coordinate of your move [0/1/2]: ")
        }
        
        let _ = io::stdin().read_line(&mut x_coord);
        
        if whos_turn == 0{
            println!("Player1, provide y coordinate of your move [0/1/2]: ");
        }else{
            println!("Player2, provide y coordinate of your move [0/1/2]: ");
        }
        
        let _ = io::stdin().read_line(&mut y_coord);

        let x_coord_int = x_coord.trim().parse::<usize>().unwrap_or(4);
        let y_coord_int = y_coord.trim().parse::<usize>().unwrap_or(4);

        if place(&mut map, x_coord_int, y_coord_int, configuration[whos_turn]){

            placed = placed + 1;

            print_map(map);

            if placed == 9{

                println!("Draw!");
                return;

            }

            if check_for_winner(map){

                if whos_turn == 0{
                    println!("Player1 won!");
                    return;
                }else{
                    println!("Player2 won!");
                    return
                }
                
            }

            if whos_turn == 1{
                whos_turn = 0;
            }else{
                whos_turn = 1;
            }


        }else{
            println!("Wrong coordinates! Try again!");
        }

    }

}

fn main() {
    
    let mut player_decision = String::new();
    
    loop {

        game();

        println!("Do you want to play again? [y/n]: ");
        let _ = io::stdin().read_line(&mut player_decision);

        if player_decision.trim() != "y"{
            break;
        }

    }
    
}
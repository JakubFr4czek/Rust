#[derive(Debug)]
enum Direction{
    Up,
    Down
}

#[derive(Debug)]
enum UIEvent{

    ButtonClicked,
    Scroll(Direction),
    KeyPressed(char)

}

use UIEvent::*;

fn describe(event : &UIEvent){

    println!("{:?}", event);

}

impl UIEvent{

    fn describe(&self){
        println!("{:?}", self);
    }

    fn call(&self){

        match self {

            ButtonClicked => println!("Button clicked!"),
            Scroll(x) => println!("Scroll value {:?}", x),
            KeyPressed(ch) => {

                let up_ch = ch.to_uppercase();
                println!("{} pressed", up_ch);
                println!("{} pressed", up_ch);

            }

        }

    }

}

fn main(){

    let clicked = UIEvent::ButtonClicked;
    let scrl = Scroll(Direction::Up);

    describe(&clicked); // Without & describe takes ownership of clicked
    clicked.describe();
    describe(&scrl);
    scrl.describe();
    describe(&clicked);
    clicked.describe();

    let a_pressed = KeyPressed('a');

    clicked.call();
    scrl.call();
    a_pressed.call();

    

}
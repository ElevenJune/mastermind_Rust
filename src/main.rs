use rand::Rng;
use std::io;

const NUMBER_OF_PEGS : usize = 4;
const NUMBER_OF_COLORS : i32 = 6;
const MAX_TURN : i32 = 12;
//QUESTION : How to have a static default constructor for GamePeg ?
//A call to GamePeg::default() does not copile : error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
static DEFAULT_HAND : [GamePeg;NUMBER_OF_PEGS] = [GamePeg{color:GamePegColor::Unknown,state:CodePeg::Wrong};NUMBER_OF_PEGS as usize];

//------------------------------------------------

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
enum GamePegColor {
    White,
    Orange,
    Blue,
    Red,
    Yellow,
    Green,
    Unknown
}

impl GamePegColor {
    fn from_i32(value: i32) -> GamePegColor {
        match value {
            0 => GamePegColor::White,
            1 => GamePegColor::Orange,
            2 => GamePegColor::Blue,
            3 => GamePegColor::Red,
            4 => GamePegColor::Yellow,
            5 => GamePegColor::Green,
            _ => GamePegColor::Unknown
        }
    }

    fn from_str(value: &str) -> GamePegColor {
        let lower_cased : String = value.to_lowercase();
        match lower_cased.trim() {
            "white" => GamePegColor::White,
            "orange" => GamePegColor::Orange,
            "blue" => GamePegColor::Blue,
            "red" => GamePegColor::Red,
            "yellow" => GamePegColor::Yellow,
            "green" => GamePegColor::Green,
            _ => GamePegColor::Unknown
        }
    }
}

//------------------------------------------------

//QUESTION : Why is this not a default Trait ?
#[derive(Copy, Clone)]
enum CodePeg {
    Correct,
    Missplaced,
    Wrong
}

#[derive(Copy, Clone)]
struct GamePeg {
    color : GamePegColor,
    state : CodePeg
}

impl Default for GamePeg {
    fn default() -> Self {
        GamePeg {
            color:GamePegColor::Unknown,
            state:CodePeg::Wrong
        }
    }
}

//------------------------------------------------


fn main() {
    //Generate master's hand
    let mut rng = rand::thread_rng();
    let master:[GamePegColor;NUMBER_OF_PEGS] = [(); NUMBER_OF_PEGS].map(|_| GamePegColor::from_i32(rng.gen_range(0..NUMBER_OF_COLORS)));
    let mut count = 1;
    print_rules();
    //print_line(master); //Debug only

    loop {
        println!("--- TURN n°{} ---",count);
        let (mut user_hand, stop) = get_user_hand();
        if stop { break;}

        let same_hands = compare_hands(&master,&mut user_hand);
        if same_hands {
            println!("You won ! The solution was :");
            print_line(master);
            break;
        }
        else if count == MAX_TURN {
            println!("You lost ! The solution was :");
            print_line(master);
            break;
        }
        count+=1;
    }
    
}

//Prints [GamePegColor,NUMBER_OF_PEGS]
fn print_line(list:[GamePegColor;NUMBER_OF_PEGS]) {
    println! ("{:?}", list);
}

//Returns user's hand as [GamePeg;NUMBER_OF_PEGS] and a stop bool that is true is the user asked to quit
fn get_user_hand() -> ([GamePeg;NUMBER_OF_PEGS],bool) {
    //Get user input
    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().eq("quit"){
            return (DEFAULT_HAND,true);
        }

        let (user_hand, error) = parse_user_input(&guess);

        if !error {
            return (user_hand,false);
        } else {
            println!("You should enter {} colors (white, orange, blue, green, yellow or red)",NUMBER_OF_PEGS);
            println!("Type \"quit\" if you wish to stop the game");
            guess.clear();
        }
    }
}

//Parses the user hand, from a ¹str to a list of GamePeg
fn parse_user_input(guess:&str) -> ([GamePeg;NUMBER_OF_PEGS],bool) {
    let split = guess.split(' ');
    let mut array = [GamePeg::default();NUMBER_OF_PEGS];

    //Parse each number, stop if there are letters or out of range numbers
    let mut i = 0;
    for s in split {
        let peg_color : GamePegColor = GamePegColor::from_str(s);

        //QUESTION : When to use match ? when to use this ?
        if peg_color == GamePegColor::Unknown {return (DEFAULT_HAND,true);}

        if i >= NUMBER_OF_PEGS{
            break;
        }
        array[i].color=peg_color;
        i+=1;
    }
    let len_ok : bool = i >= NUMBER_OF_PEGS;
    if len_ok {
        return (array,false)
    } else 
    {
        return (DEFAULT_HAND,true)
    };
}

//Compares the user'hand with the master's code and returns true if they are the same
//QUESTION : Is it better to have the state of a peg in the GamePeg struct ? It has to be passed as &mut, which seems more complicated
fn compare_hands(master:&[GamePegColor;NUMBER_OF_PEGS],user:&mut[GamePeg;NUMBER_OF_PEGS]) -> bool{
    let len = master.len();
    let mut master_treated : [i32;NUMBER_OF_PEGS] = [0; NUMBER_OF_PEGS];
    let mut user_treated : [i32;NUMBER_OF_PEGS] = [0; NUMBER_OF_PEGS];

    //Computes correct
    for i in 0..len {
        if user[i].color==master[i] && master_treated[i]==0 {
            user[i].state=CodePeg::Correct;
            master_treated[i]=1;
            user_treated[i]=1;
        }
    }
    
    //Computes missplaced
    for i in 0..len {
        for j in 0..len{
            if master_treated[j]==0 && user_treated[i]==0 && user[i].color==master[j] {
                user[i].state=CodePeg::Missplaced;
                master_treated[j]=1;
                user_treated[i]=1;
            }
        }
    }

    return check_if_finished(user);
}

//QUESTION : seems less performant as before since I have to iterate through the list again, but is it cleaner ?
fn check_if_finished(user:&[GamePeg;NUMBER_OF_PEGS]) -> bool{
    let len = user.len();
    let mut correct = 0;
    let mut missplaced = 0;
    for i in 0..len{
        match user[i].state {
            CodePeg::Correct=>correct+=1,
            CodePeg::Missplaced=>missplaced+=1,
            _ => {}
        }
    }
    let finished = correct==NUMBER_OF_PEGS;
    if !finished{
        println!("N° of correct pegs: {}, n° of missplaced pegs: {}", correct, missplaced);
    }
    finished    
}

fn print_rules() {
    let mut format = String::new();
    for i in 0..NUMBER_OF_PEGS{
        let num = format!("white{}", if i!=NUMBER_OF_PEGS-1 {" "} else {""});
        format.push_str(&num);
    }
    println!("--- Mastermind ---");
    println!("- Rules :");
    println!("-     Find the master's code, made up of {} colors (white, orange, blue, green, yellow or red)",NUMBER_OF_PEGS);
    println!("-     You have 12 tries, if you fail to discover the master's code in less than 12 tries you loose");
    println!("-     At each turn, you enter your guess in the format \"{}\" (your colors are to be seperated by a space)",format);
    println!("-     The master will give you a hint about how close is your guess from his code with pins :");
    println!("-         - A correct peg means that one of the peg you chose is of the right color and is in the right spot");
    println!("-         - A missplaced peg means that one of the peg you chose is of the right color, but not at the right spot");
    println!("-     Good luck !");
    println!("You should enter {} colors (white, orange, blue, green, yellow or red)",NUMBER_OF_PEGS);
    println!("Type \"quit\" if you wish to stop the game");
    println!("Please enter your first guess :")
}
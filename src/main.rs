use rand::Rng;
use std::io;

const NUMBER_OF_PINS : i32 = 4;
const NUMBER_OF_COLORS : i32 = 6;
const MAX_TURN : i32 = 12;
static DEFAULT_HAND : [i32;NUMBER_OF_PINS as usize] = [0;NUMBER_OF_PINS as usize];


fn main() {

    //Generate master's hand
    let mut rng = rand::thread_rng();
    let master:[i32;NUMBER_OF_PINS as usize] = [(); NUMBER_OF_PINS as usize].map(|_| rng.gen_range(1..NUMBER_OF_COLORS+1));
    let mut count = 1;
    print_rules();
    //println!("Solution :");
    //print_line(master);

    loop {
        println!("--- TURN n°{} ---",count);
        let (user_hand, stop) = get_user_hand();
        if stop { break;}

        let (red_pins, white_pins) = compare_hands(&master,&user_hand);
        println!("Red pins: {}, white pins: {}", red_pins, white_pins);
        if red_pins == NUMBER_OF_PINS {
            println!("You won ! The solution was :");
            print_line(master);
            break;
        }
        if count == MAX_TURN {
            println!("You lost ! The solution was :");
            print_line(master);
            break;
        }
        count+=1;
    }
    
}

//Prints [i32,NUMBER_OF_PINS]
fn print_line(list:[i32;NUMBER_OF_PINS as usize]) {
    println! ("{:?}", list);
}

//Returns user's hand as [i32;NUMBER_OF_PINS] and a stop bool that is true is the user asked to quit
fn get_user_hand() -> ([i32;NUMBER_OF_PINS as usize],bool) {
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
            println!("You should enter {} digits, seperated by a space, in the range 1 - {}",NUMBER_OF_PINS, NUMBER_OF_COLORS);
            println!("Type \"quit\" if you wish to stop the game");
            guess.clear();
        }
    }
}

fn parse_user_input(guess:&str) -> ([i32;NUMBER_OF_PINS as usize],bool) {
    let split = guess.split(' ');
    let mut guess_int : Vec<i32> = vec![];

    //Parse each number, stop if there are letters or out of range numbers
    for s in split {
        let num:i32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => {return (DEFAULT_HAND,true);},
        };
        if num <= 0 || num >NUMBER_OF_COLORS {
            return (DEFAULT_HAND,true);
        }

        guess_int.push(num);
        if guess_int.len() >= NUMBER_OF_PINS as usize{
            break;
        }
    }
    let len_ok : bool = guess_int.len()>=NUMBER_OF_PINS as usize;
    if len_ok {
        let mut array = [0;NUMBER_OF_PINS as usize];
        for i in 0..array.len() {
            array[i]=guess_int[i];
        }
        return (array,false)
    } else 
    {
        return (DEFAULT_HAND,true)
    };
}

//2 if good color in right spot
//1 if good color but not good spot
//0 if not good color
fn compare_hands(master:&[i32;NUMBER_OF_PINS as usize],user:&[i32;NUMBER_OF_PINS as usize]) -> (i32,i32){
    let len = master.len();
    let mut red_pin = 0;
    let mut white_pin = 0;
    let mut master_treated : [i32;NUMBER_OF_PINS as usize] = [0; NUMBER_OF_PINS as usize];
    let mut user_treated : [i32;NUMBER_OF_PINS as usize] = [0; NUMBER_OF_PINS as usize];

    //Compute red pins
    for i in 0..len {
        if user[i]==master[i] && master_treated[i]==0 {
            red_pin+=1;
            master_treated[i]=1;
            user_treated[i]=1;
        }
    }
    
    //Compute white pins
    for i in 0..len {
        for j in 0..len{
            if master_treated[j]==0 && user_treated[i]==0 && user[i]==master[j] {
                white_pin+=1;
                master_treated[j]=1;
                user_treated[i]=1;
            }
            //print_line(master_treated);
        }
    }

    (red_pin, white_pin)
}

fn print_rules() {
    println!("--- Mastermind ---");
    println!("- Rules :");
    println!("-     Find the master's code, made up of 5 digits from 1 to 8 ");
    println!("-     You have 12 tries, if you fail to discover the master's code in less than 12 tries you loose");
    println!("-     At each turn, you enter your guess in the format \"1 2 3 4 5\" (your digits are to be seperated by a space)");
    println!("-     The master will give you a hint about how close is your guess from his code with pins :");
    println!("-         - A red pin means that one of the digit you chose is the right one and is in the right spot");
    println!("-         - A white pin means that one of the digit you chose is present in his code, but not at the right spot");
    println!("-     Good luck !");
    println!("You should enter {} digits, seperated by a space, in the range 1 - {}",NUMBER_OF_PINS, NUMBER_OF_COLORS);
    println!("Type \"quit\" if you wish to stop the game");
    println!("Please enter your first guess :")
}
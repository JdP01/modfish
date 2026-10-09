#![allow(dead_code)]
use std::io::{self, Write};
use regex::Regex;
use std::collections::HashMap;


struct Boards{ 
    pawns: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    king: u64,
    queen: u64,
    occupancy: u64,
}

struct Side{ 
    color: bool,
    pieces: Boards,
}

fn main() {
    
    let white_board = Boards{pawns:0xFF00, //board starting positions for white
        rooks:0x81,
        bishops:0x24,
        knights:0x42,
        king:0x10,
        queen:0x8,
        occupancy:0xFFFF
    };
    let black_board = Boards{pawns:0xFF000000000000, //same for black
        rooks:0x8100000000000000,
        bishops:0x2400000000000000,
        knights:0x4200000000000000,
        king:0x1000000000000000,
        queen:0x800000000000000,
        occupancy:0xFFFF000000000000
    };
    let white = Side{color: true,pieces: white_board}; //team color 
    let black = Side{color: false,pieces: black_board};

    game_loop(white,black);
}


fn game_loop(white: Side,black: Side){ 

    let done: bool = false;

    while !done{ 
        let mut from: String; 
        let mut to: String; 

        //WHITE TURN
        (from,to) = game_dialog("White".to_string());
        //USE (from,to) for interpret move and actually execute it for black
        println!("White from:  {}",from);
        println!("White to  :  {}",to);

        //here we have to implement the call that actually moves the pawn 
        let (start_mask,end_mask) = interpret_move(from,to);
        

        print_board(&white,&black);

        //BLACKS TURN
        (from,to) = game_dialog("Black".to_string());
        //USE (from,to) for interpret move and actually execute it for black
        println!("Black from:  {}",from);
        println!("Black to  :  {}",to);

        print_board(&white,&black);


    }
}

fn interpret_move(input_from:String ,input_to: String) -> (u64,u64){ 


    //array with a = 0, up to H = 7
    let file_index:[u64;8] = [0x1,0x2,0x4,0x8,0x10,0x20,0x40,0x80];
    
    let starting_bytes = input_from.as_bytes();// Convert input to a byte slice so we can index it instantly
    let starting_file = (starting_bytes[0] - b'a') as u32; // we subtract the ascii value of 'a' to get a 0 indexed number for the letters between a-h    
    let starting_rank = (starting_bytes[1] - b'0') as u32; //we subtract the ascii value of '0' toget a 0 indexed number for the values between 0 and 8

    let ending_bytes = input_to.as_bytes();// Convert input to a byte slice so we can index it instantly
    let ending_file = (ending_bytes[0] - b'a') as u32; // we subtract the ascii value of 'a' to get a 0 indexed number for the letters between a-h    
    let ending_rank = (ending_bytes[1] - b'0') as u32; //we subtract the ascii value of '0' toget a 0 indexed number for the values between 0 and 8


    //from mask is the starting letter shifted by 8*starting_rank
    let from_mask: u64 = file_index[starting_file as usize] << starting_rank;    
    let to_mask: u64 = file_index[ending_file as usize] << ending_rank; 


    return (from_mask,to_mask);
}

fn game_dialog(color:String)-> (String,String){  

    //currently regex only checks that the square exists
    //it doesnt check whether we have a valid move from move generation

    let mut from = String::new();
    let mut to = String::new();
    let re = Regex::new(&r"(?i)^[a-h][1-8]$").unwrap();

    println!("{} to move: ",color);

    //GET WHITE MOVES INPUT
    //loop isn't expected to run more than once its just to only get valid inputs 
    loop {
        from.clear();
        print!("from: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut from).expect("error: unable to read user input");
        
        from = from.trim().to_string(); //remove newline \n 
        
        if re.is_match(&from.trim()){ //if regex matches
            break;
        }
        else{ 
            println!("Invalid position, try again.");
        }
    }

    loop {
        to.clear();
        print!("to: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut to).expect("error: unable to read user input");
        
        to = to.trim().to_string(); //clean up input, remove \n
        
        if to == from{
            println!("invalid move, you cant go to the same square, from: {}, to: {}",from,to);
        }
        else if re.is_match(&to){ //if regex matches
            break;
        }
        else{ 
            println!("Invalid position, try again.");
        }
    }
    
    return (from,to); 
}

fn print_board(white: &Side,black: &Side)-> () {

    let mut live_board:[char; 64] = ['0','0','0','0','0','0','0','0', //empty board char arr rep
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0'];


    for i in 0..64{ // this masking method works but will not handle if two pieces are ever in the same place (which should not happen if I write the rules correctly)

        let mask1 = 0x8000000000000000 >> i;

        if mask1 & black.pieces.queen != 0{live_board[i] = '♛' }
        if mask1 & black.pieces.king != 0{live_board[i] = '♚'}
        if mask1 & black.pieces.knights != 0{live_board[i] = '♞'}
        if mask1 & black.pieces.bishops != 0{live_board[i] = '♝'}
        if mask1 & black.pieces.rooks != 0{live_board[i] = '♜'}
        if mask1 & black.pieces.pawns != 0{live_board[i] = '♟'}

        if mask1 & white.pieces.queen != 0{live_board[i] = '♕'}
        if mask1 & white.pieces.king != 0{live_board[i] = '♔'}
        if mask1 & white.pieces.knights != 0{live_board[i] = '♘'}
        if mask1 & white.pieces.bishops != 0{live_board[i] = '♗'}
        if mask1 & white.pieces.rooks != 0{live_board[i] = '♖'}
        if mask1 & white.pieces.pawns != 0{live_board[i] = '♙'}
    }

    for i in (0..64).step_by(8) { //prints it "backwards" or upsidedown or whatever since terminal prints from top to bottom

        println! (" {} {} {} {} {} {} {} {} ",live_board[i+7],live_board[i+6],live_board[i+5],live_board[i+4],live_board[i+3],live_board[i+2],live_board[i+1],live_board[i]);   

    }

}
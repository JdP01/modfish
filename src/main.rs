#![allow(dead_code)]

struct Boards{ 
    pawns: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    king: u64,
    queen: u64,
}

struct Side{ 
    color: bool,
    pieces: Boards,
}

fn main() {
    
    let white_board = Boards{pawns:0xFF00, //board starting positions for white
        rooks:0x81,
        bishops:0x42,
        knights:0x24,
        king:0x8,
        queen:0x10
    };
    let white = Side{color: true,pieces: white_board}; //team color 


    let black_board = Boards{pawns:0xFF000000000000, //same for black
        rooks:0x8100000000000000,
        bishops:0x4200000000000000,
        knights:0x2400000000000000,
        king:0x800000000000000,
        queen:0x1000000000000000
    };
    let black = Side{color: true,pieces: black_board};
   
    print_board(white,black);
}

fn print_board(white: Side,black: Side)-> () {

    let b_q = black.pieces.queen;
    let b_k = black.pieces.king;
    let b_kn = black.pieces.knights;
    let b_b = black.pieces.bishops;
    let b_r = black.pieces.rooks;
    let b_p = black.pieces.pawns;

    let w_q = white.pieces.queen;
    let w_k = white.pieces.king;
    let w_kn = white.pieces.knights;
    let w_b = white.pieces.bishops;
    let w_r = white.pieces.rooks;
    let w_p = white.pieces.pawns;

    let white :[u64; 6] = [w_q,w_k,w_kn,w_b,w_r,w_p];
    let black :[u64; 6] = [b_q,b_k,b_kn,b_b,b_r,b_p];


    for i in 0..8{
        
        
        
        println!("{:08b}",white[i]);
    }
}
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
    

    let mut live_board:[char; 64] = ['0','0','0','0','0','0','0','0', //empty board char arr rep
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0',
                                     '0','0','0','0','0','0','0','0'];


    for i in 0..64{ // this masking method works but will not handle if two pieces are ever in the same place (which should not happen if I write the rules correctly)

        let mask1 = 1 << i;

        if mask1 & b_q != 0{live_board[i] = '♛' }
        if mask1 & b_k != 0{live_board[i] = '♚'}
        if mask1 & b_kn != 0{live_board[i] = '♞'}
        if mask1 & b_b != 0{live_board[i] = '♝'}
        if mask1 & b_r != 0{live_board[i] = '♜'}
        if mask1 & b_p != 0{live_board[i] = '♟'}

        if mask1 & w_q != 0{live_board[i] = '♕'}
        if mask1 & w_k != 0{live_board[i] = '♔'}
        if mask1 & w_kn != 0{live_board[i] = '♘'}
        if mask1 & w_b != 0{live_board[i] = '♗'}
        if mask1 & w_r != 0{live_board[i] = '♖'}
        if mask1 & w_p != 0{live_board[i] = '♙'}
    }

    for i in (0..64).step_by(8) { 

        println! (" {} {} {} {} {} {} {} {} ",live_board[i],live_board[i+1],live_board[i+2],live_board[i+3],live_board[i+4],live_board[i+5],live_board[i+6],live_board[i+7]);   

    }


}
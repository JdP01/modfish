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

    //let q = white.pieces.queen;
    //let k = white.pieces.king;
    //let kn = white.pieces.knights;
    //let b = white.pieces.bishops;
    //let r = white.pieces.rooks;
    //let p = white.pieces.pawns;

    //print white chec

    let q = black.pieces.queen;
    let k = black.pieces.king;
    let kn = black.pieces.knights;
    let b = black.pieces.bishops;
    let r = black.pieces.rooks;
    let p = black.pieces.pawns;

    println!("{:064b}",p);    
    println!("{:064b}",q);
    println!("{:064b}",k);
    println!("{:064b}",kn);
    println!("{:064b}",b);
    println!("{:064b}",r);
    
    println!("{:064b}",q | k | kn | b | r | p);

 
    
    print_board()
}

fn print_board()-> () {
    println!("hello");
    
}
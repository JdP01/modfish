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
    
    let white_board = Boards{pawns:0xFF00,
        rooks:0x8100000000000000,
        bishops:0,
        knights:0,
        king:0,
        queen:0};
    let white = Side{color: true,pieces: white_board};

    let black_board = Boards{pawns:0xFF000000000000,
        rooks:0x1000000000000000,
        bishops:0,
        knights:0,
        king:0,
        queen:0};
    let black = Side{color: true,pieces: black_board};

    println!("{:064b}",white.pieces.pawns);
    println!("{:064b}",white.pieces.rooks);
    println!("{:064b}",black.pieces.pawns);

    

}

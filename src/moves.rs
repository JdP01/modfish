
//Create board masks for all valid moves of a pawn
fn pawn_valid_move_mask(white: &Side, black: &Side)->(u64,u64) { 


    //Observations, I calculate the one fwd move on first rank pawns on itself and again on all other pawns
    // I can instead, calculate the one square move identically for 

    let mut start_rank_mask:u64 = 0xFF00;
    let mut pawns = white.pieces.pawns;
    //get pawns on first rank to generate their valid moves (1 and 2 squares forward)
    //later we get the mask of all other pawns that can only move forward once
    let mut starting_rank_pawns:u64 = (start_rank_mask & pawns); 
    let mut other_pawns:u64 = (!start_rank_mask & pawns);
    let mut first_rank_valid_moves = move_north_k(starting_rank_pawns,1)|move_north_k(starting_rank_pawns,2);
    let mut other_valid_moves = move_north_k(other_pawns,1);

    //remove first rank mask removes all the pieces behind the starting pawns so the spots in that row are never valid
    let mut remove_first_rank_mask:u64 = !0xFF;
    let white_valid_moves = remove_first_rank_mask & ((first_rank_valid_moves & other_valid_moves) ^ white.pieces.occupancy ^ white.pieces.occupancy); 


    //CREATE THE SAME MASK FOR BLACK PAWNS
    //------------------------------------
    start_rank_mask = 0xFF000000000000;
    pawns = black.pieces.pawns;
    //get pawns on first rank to generate their valid moves (1 and 2 squares forward)
    //later we get the mask of all other pawns that can only move forward once
    starting_rank_pawns = (start_rank_mask & pawns); 
    other_pawns = (!start_rank_mask & pawns);

    first_rank_valid_moves = move_south_k(starting_rank_pawns,1)|move_south_k(starting_rank_pawns,2);
    other_valid_moves = move_south_k(other_pawns,1);

    //remove first rank mask removes all the pieces behind the starting pawns so the spots in that row are never valid
    remove_first_rank_mask = !0xFF00000000000000;
    let black_valid_moves = remove_first_rank_mask & ((first_rank_valid_moves & other_valid_moves) ^ white.pieces.occupancy ^ white.pieces.occupancy); 

    return (white_valid_moves,black_valid_moves);
}

fn move_pawn_fwd(white: &Side, start_mask: u64, end_mask: u64)->(bool) { 

    //this can only handle forward moves
    let mut start_rank_mask:u64 = 0xFF00;
    let move_mask = start_mask & end_mask; 
    let pawns: u64 = white.pieces.pawns;
    let white_space: u64 = white.pieces.occupancy;
    let black_space: u64 = black.pieces.occupancy;
    let taken= white_space & black_space;

    let pawn = pawns & start_mask; //the pawn we want to move
    if pawn == 0 { println!("pawn = 0"); return -1;} //pawn doesnt exist or start mask in wrong place

    //white & black space gives us all the full squares
    // all valid 1 space moves for our pawn
    let valid_fwd_moves: u64 = move_north_k(pawn,1) ^ taken;

    if pawn & start_rank_mask: //if pawn at the starting rank add ability to move 2
        // = ability to move 1 + ability to move 2
        valid_fwd_moves |= (move_north_k(valid_fwd_moves,1) ^ taken);

    if valid_fwd_moves & end_mask == end_mask { //if the place we are moving to is within our valid moves
        //successful move
        white.pieces.occupancy ^= move_mask;
        white.pieces.pawns ^= move_mask;
        return True;
    }
    
}

fn pawn_take(){ 
    //mask for taking diagonally with enemy pieces
    let valid_takes: u64 = ((move_nw_k(pawn,1) | move_ne_k(pawn,1))^white_space) & black_space;

}







//MIGHT NEED CONDITIONAL CHECKS TO MAKE SURE OUR PIECE DOESNT GO OFF THE BOARD 
fn move_north_k(piece:u64,k:u16) -> (u64) {  return(piece << (8*k));}
fn move_south_k(piece:u64,k:u16) -> (u64) {  return(piece >> (8*k));}
fn move_east_k(piece:u64,k:u16) -> (u64) {  return(piece << (1*k));}
fn move_west_k(piece:u64,k:u16) -> (u64) {  return(piece >> (1*k));}
fn move_nw_k(piece:u64,k:u16) -> (u64) {  return(piece << (9*k));}
fn move_ne_k(piece:u64,k:u16) -> (u64) {  return(piece << (7*k));}
fn move_sw_k(piece:u64,k:u16) -> (u64) {  return(piece >> (7*k));}
fn move_se_k(piece:u64,k:u16) -> (u64) {  return(piece >> (9*k));}
use crate::types::{Bitboard, Square};

// general utility methods used throughout the program

// nicely displays the bitboard to look like the chessboard
#[allow(dead_code)]
pub fn display_bitboard(bitboard: Bitboard) {
    for rank in bitboard.to_be_bytes() {
        for i in (0..8).rev() {
            print!("{} ", (rank >> i) & 1);
        }
        println!();
    }
}

// converts a string in algebraic notation (ex: b4) to integer type
pub fn square_from_algebraic(algebraic: &str) -> Option<Square> {
    let file: Square = match algebraic.chars().nth(0)? {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return None,
    };

    let rank: Square = match algebraic.chars().nth(1)? {
        '8' => 0,
        '7' => 1,
        '6' => 2,
        '5' => 3,
        '4' => 4,
        '3' => 5,
        '2' => 6,
        '1' => 7,
        _ => return None,
    };

    Some((rank * 8) + file)
}

// used in move generation for bounds checking, can be bitwise AND-ed with piece position to mask out pieces on a certain file
pub struct FileBoundMask;
impl FileBoundMask {
    pub const A: Bitboard = 0x7F_7F_7F_7F_7F_7F_7F_7F;
    pub const B: Bitboard = 0xBF_BF_BF_BF_BF_BF_BF_BF;
    // ...
    pub const G: Bitboard = 0xFD_FD_FD_FD_FD_FD_FD_FD;
    pub const H: Bitboard = 0xFE_FE_FE_FE_FE_FE_FE_FE;
}

mod alpha_beta;
mod evaluate;
mod ordering;
mod tables;
mod tt;

pub use alpha_beta::{best_move, mate_in, SearchTT};

/// Represents the score of the board, where a positive number implies moving side is ahead
pub type Score = i16;

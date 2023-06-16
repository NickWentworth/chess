mod board;
mod core;
mod evaluate;
mod move_generator;
mod tests;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    let mut b = board::Board::new(fen);
    let mg = move_generator::MoveGenerator::new();

    println!("{}", tests::perft(&mg, &mut b, 5));

    println!("{}", evaluate::evaluate(b.position()));
}

use wasm_bindgen::prelude::*;

use chers::Position;

#[wasm_bindgen]
pub fn get_best_move(fen: &str, depth: u32) -> String {
    let fen = format!("{} b KQkq - 0 2", fen);
    let mut pos = Position::from_fen(&fen).unwrap();
    let m = pos.search(depth);

    if m.is_promotion() {
        format!("{}{}{}", m.origin(), m.target(), m.promotion_piece())
    } else {
        format!("{}{}", m.origin(), m.target())
    }
}

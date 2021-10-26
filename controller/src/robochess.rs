use crate::hbot::HBot;
use chers::BitMove;
use chers::Color;
use chers::ParsedMove;
use chers::Piece;
use chers::PieceType;
use chers::Position;
use chers::Square;

const X_OFFSET: i32 = 100;
const Y_OFFSET: i32 = 0;
const Y_PLACEMENT_OFFSET: i32 = 50;
const SIZE_OFF_SQUARE: i32 = 264;

fn get_square_coordinates(sq: Square) -> (i32, i32) {
    let rank = sq.rank().to_i32();
    let file = sq.file().to_i32();
    let x = X_OFFSET + (7 - file) * SIZE_OFF_SQUARE + SIZE_OFF_SQUARE / 2;
    let y = Y_OFFSET + rank * SIZE_OFF_SQUARE + SIZE_OFF_SQUARE / 2;
    (x, y)
}

pub struct RoboChess {
    pub controller: HBot,
    position: Position,
    captured_pieces_white: Vec<Piece>,
    captured_pieces_black: Vec<Piece>,
}

impl RoboChess {
    pub fn new() -> Self {
        let controller = HBot::new(27, 17, 6, 26, 16, 5, 13);
        let position = Position::new();
        Self {
            controller,
            position,
            captured_pieces_white: Vec::new(),
            captured_pieces_black: Vec::new(),
        }
    }

    /// Returns false if the move is illegal
    pub fn make_move(&mut self, parsed_move: ParsedMove) -> bool {
        // get the BitMove corresponding to the move, because the bit_move carries more information
        let mut m = BitMove::NULL;
        let moves = self.position.generate_legal_moves();
        for bit_move in moves {
            if bit_move == parsed_move {
                m = bit_move;
                break;
            }
        }
        if m == BitMove::NULL {
            // illegal move
            return false;
        }

        if m.is_capture() {
            let capture_square = if m.is_en_passant() {
                if self.position.side_to_move() == Color::WHITE {
                    Square::new(m.target().file(), m.target().rank() - 1)
                } else {
                    Square::new(m.target().file(), m.target().rank() + 1)
                }
            } else {
                m.target()
            };
            self.capture_piece(capture_square);
        }

        if m.is_castle() {
            todo!();
        }

        if m.is_promotion() {
            todo!();
        }

        match self.position.get_square(m.origin()).piece_type() {
            PieceType::KNIGHT => {
                let (ox, oy) = get_square_coordinates(m.origin());
                let (tx, ty) = get_square_coordinates(m.target());
                let dx = tx - ox;
                let dy = ty - oy;
                dbg!(dx, dy);
                self.move_to_square_fast(m.origin());
                self.controller.up();
                self.controller.wait();
                if dx.abs() < dy.abs() {
                    dbg!("1");
                    self.controller.move_to_xy_slow(ox + dx / 2, oy);
                    self.controller.wait();
                    self.controller.move_to_xy_slow(ox + dx / 2, oy + dy);
                    self.controller.wait();
                    self.controller.move_to_xy_slow(ox + dx, oy + dy);
                    self.controller.wait();
                } else {
                    dbg!("2");
                    self.controller.move_to_xy_slow(ox, oy + dy / 2);
                    self.controller.move_to_xy_slow(ox + dx, oy + dy / 2);
                    self.controller.move_to_xy_slow(ox + dx, oy + ox);
                }
                self.controller.down();
                self.controller.wait();
            }
            _ => {
                self.move_to_square_fast(m.origin());
                self.controller.up();
                self.controller.wait();
                let (x, mut y) = get_square_coordinates(m.target());
                y += Y_PLACEMENT_OFFSET;
                self.controller.move_to_xy_slow(x, y);
                self.controller.down();
                self.controller.wait();
            }
        }
        self.position.make_bit_move(m);
        true
    }

    pub fn capture_piece(&mut self, sq: Square) {
        self.move_to_square_fast(sq);
        self.controller.up();
        self.controller.wait();
        let (mut x, mut y) = get_square_coordinates(sq);

        // capture white piece
        if !self.position.side_to_move() == Color::WHITE {
            y += SIZE_OFF_SQUARE / 2;
            self.controller.move_to_xy_slow(x, y);
            x = X_OFFSET - SIZE_OFF_SQUARE / 4;
            self.controller.move_to_xy_slow(x, y);
            y = self.captured_pieces_white.len() as i32 * SIZE_OFF_SQUARE / 2 + SIZE_OFF_SQUARE / 4;
            self.captured_pieces_white
                .push(self.position.get_square(sq));
        } else {
            // capture black piece
            y -= SIZE_OFF_SQUARE / 2;
            self.controller.move_to_xy_slow(x, y);
            x = X_OFFSET + 33 * SIZE_OFF_SQUARE / 4;
            self.controller.move_to_xy_slow(x, y);
            y = 8 * SIZE_OFF_SQUARE - self.captured_pieces_white.len() as i32 * SIZE_OFF_SQUARE / 2
                + SIZE_OFF_SQUARE / 4;
            self.captured_pieces_black
                .push(self.position.get_square(sq));
        }
        self.controller.move_to_xy_slow(x, y);
        self.controller.down();
        self.controller.wait();
    }

    pub fn move_to_square_slow(&mut self, sq: Square) {
        let (x, y) = get_square_coordinates(sq);
        self.controller.move_to_xy_slow(x, y);
    }

    pub fn move_to_square_fast(&mut self, sq: Square) {
        let (x, y) = get_square_coordinates(sq);
        self.controller.move_to_xy_fast(x, y);
    }
}

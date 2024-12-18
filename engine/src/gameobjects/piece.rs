
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}


impl Piece {
    pub fn to_unicode(&self) -> &'static str {
        match (self.color, self.piece_type) {
            (Color::White, PieceType::King) => "♚",
            (Color::White, PieceType::Queen) => "♛",
            (Color::White, PieceType::Rook) => "♜",
            (Color::White, PieceType::Bishop) => "♝",
            (Color::White, PieceType::Knight) => "♞",
            (Color::White, PieceType::Pawn) => "♟",
            (Color::Black, PieceType::King) => "♚",
            (Color::Black, PieceType::Queen) => "♛",
            (Color::Black, PieceType::Rook) => "♜",
            (Color::Black, PieceType::Bishop) => "♝",
            (Color::Black, PieceType::Knight) => "♞",
            (Color::Black, PieceType::Pawn) => "♟",
        }
    }
}
use crate::gameobjects::piece::{
    Piece,
    Color,
    PieceType
};


pub struct Board {
    pub board: [[Option<Piece>; 8]; 8]
}


impl Board {
    pub fn new() -> Self {
        let mut board = [[None; 8]; 8];

        // Initialize the board with pieces
        board[0] = [
            Some(Piece { piece_type: PieceType::Rook, color: Color::White }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
            Some(Piece { piece_type: PieceType::Queen, color: Color::White }),
            Some(Piece { piece_type: PieceType::King, color: Color::White }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
            Some(Piece { piece_type: PieceType::Rook, color: Color::White }),
        ];
        board[1] = [
            Some(Piece { piece_type: PieceType::Pawn, color: Color::White }); 8
        ];

        board[7] = [
            Some(Piece { piece_type: PieceType::Rook, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Queen, color: Color::Black }),
            Some(Piece { piece_type: PieceType::King, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Rook, color: Color::Black }),
        ];
        board[6] = [
            Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }); 8
        ];

        Self { board }
    }
}
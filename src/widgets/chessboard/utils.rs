use pleco::{File, Piece, Rank, SQ};

pub fn get_piece_type_from(piece: Piece) -> char {
    match piece {
        Piece::WhitePawn => 'P',
        Piece::WhiteKnight => 'N',
        Piece::WhiteBishop => 'B',
        Piece::WhiteRook => 'R',
        Piece::WhiteQueen => 'Q',
        Piece::WhiteKing => 'K',

        Piece::BlackPawn => 'p',
        Piece::BlackKnight => 'n',
        Piece::BlackBishop => 'b',
        Piece::BlackRook => 'r',
        Piece::BlackQueen => 'q',
        Piece::BlackKing => 'k',

        _ => ' ',
    }
}

pub fn get_uci_move_for(start: SQ, end: SQ, promotion: Option<char>) -> String {
    format!(
        "{}{}{}",
        square_to_algebraic(start),
        square_to_algebraic(end),
        promotion_as_algebraic(promotion)
    )
}

pub fn is_side_piece(piece: Piece, white_turn: bool) -> bool {
    match piece {
        Piece::None => false,
        Piece::WhitePawn => white_turn,
        Piece::WhiteKnight => white_turn,
        Piece::WhiteBishop => white_turn,
        Piece::WhiteRook => white_turn,
        Piece::WhiteQueen => white_turn,
        Piece::WhiteKing => white_turn,
        Piece::BlackPawn => !white_turn,
        Piece::BlackKnight => !white_turn,
        Piece::BlackBishop => !white_turn,
        Piece::BlackRook => !white_turn,
        Piece::BlackQueen => !white_turn,
        Piece::BlackKing => !white_turn,
    }
}

fn square_to_algebraic(square: SQ) -> String {
    let file = square.file();
    let file = String::from(match file {
        File::A => "a",
        File::B => "b",
        File::C => "c",
        File::D => "d",
        File::E => "e",
        File::F => "f",
        File::G => "g",
        File::H => "h",
    });

    let rank = square.rank();
    let rank = String::from(match rank {
        Rank::R1 => "1",
        Rank::R2 => "2",
        Rank::R3 => "3",
        Rank::R4 => "4",
        Rank::R5 => "5",
        Rank::R6 => "6",
        Rank::R7 => "7",
        Rank::R8 => "8",
    });

    format!("{}{}", file, rank)
}

fn promotion_as_algebraic(piece: Option<char>) -> String {
    match piece {
        Some(piece) => match piece {
            'q' | 'r' | 'b' | 'n' => format!("{}", piece),
            _ => String::from(""),
        },
        None => String::from(""),
    }
}

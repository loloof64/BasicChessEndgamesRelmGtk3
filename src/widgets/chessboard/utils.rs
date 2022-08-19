use owlchess::{Color, Piece};

pub fn get_piece_type_from(piece: Piece, color: Color) -> char {
    match piece {
        Piece::Pawn => {
            if color == Color::White {
                'P'
            } else {
                'p'
            }
        }
        Piece::Knight => {
            if color == Color::White {
                'N'
            } else {
                'n'
            }
        }
        Piece::Bishop => {
            if color == Color::White {
                'B'
            } else {
                'b'
            }
        }
        Piece::Rook => {
            if color == Color::White {
                'R'
            } else {
                'r'
            }
        }
        Piece::Queen => {
            if color == Color::White {
                'Q'
            } else {
                'q'
            }
        }
        Piece::King => {
            if color == Color::White {
                'K'
            } else {
                'k'
            }
        }
    }
}

pub fn get_uci_move_for(
    start_file: u8,
    start_rank: u8,
    end_file: u8,
    end_rank: u8,
    promotion: Option<char>,
) -> String {
    format!(
        "{}{}{}",
        square_coords_to_algebraic(start_file, start_rank),
        square_coords_to_algebraic(end_file, end_rank),
        promotion_as_algebraic(promotion)
    )
}

fn square_coords_to_algebraic(file: u8, rank: u8) -> String {
    let file = String::from(match file {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Forbidden file value : {}.", file),
    });

    let rank = String::from(match rank {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        _ => panic!("Forbidden rank value : {}.", rank),
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

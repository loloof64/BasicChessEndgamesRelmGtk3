use owlchess::{Color, Cell, Coord, File, Piece, Rank};
use owlchess::moves::PromotePiece;
use owlchess::moves::uci;

pub(crate) fn get_piece_type_from(piece: Piece, color: Color) -> char {
    Cell::from_parts(color, piece).as_char()
}

pub(crate) fn get_uci_move_for(
    start_file: u8,
    start_rank: u8,
    end_file: u8,
    end_rank: u8,
    promotion: Option<char>,
) -> uci::Move {
    uci::Move::Move {
        src: parse_square_coords(start_file, start_rank),
        dst: parse_square_coords(end_file, end_rank),
        promote: parse_promotion(promotion),
    }
}

fn parse_square_coords(file: u8, rank: u8) -> Coord {
    let file = match file {
        0 => File::A,
        1 => File::B,
        2 => File::C,
        3 => File::D,
        4 => File::E,
        5 => File::F,
        6 => File::G,
        7 => File::H,
        _ => panic!("Forbidden file value : {}.", file),
    };

    let rank = match rank {
        0 => Rank::R1,
        1 => Rank::R2,
        2 => Rank::R3,
        3 => Rank::R4,
        4 => Rank::R5,
        5 => Rank::R6,
        6 => Rank::R7,
        7 => Rank::R8,
        _ => panic!("Forbidden rank value : {}.", rank),
    };

    Coord::from_parts(file, rank)
}

fn parse_promotion(piece: Option<char>) -> Option<PromotePiece> {
    Some(match piece? {
        'q' => PromotePiece::Queen,
        'r' => PromotePiece::Rook,
        'b' => PromotePiece::Bishop,
        'n' => PromotePiece::Knight,
        other => panic!("Forbidden promote value : {}.", other),
    })
}

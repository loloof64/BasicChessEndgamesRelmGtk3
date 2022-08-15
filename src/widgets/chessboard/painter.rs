use super::ChessBoard;
use gtk::{cairo::Context, prelude::GdkContextExt};
use pleco::{Board, Piece, SQ};

pub struct Painter {}

impl Painter {
    pub fn clear_background(cx: &Context, size: f64) {
        cx.set_source_rgb(0.3, 0.3, 0.8);
        cx.rectangle(0.0, 0.0, size, size);
        cx.fill().unwrap();
    }

    pub fn paint_cells(cx: &Context, cells_size: f64) {
        for row in 0..8 {
            for col in 0..8 {
                let navajowhite = (1.0, 0.87, 0.68);
                let peru = (0.8, 0.52, 0.25);
                let is_white_cell = (row + col) % 2 == 0;
                let background_color = if is_white_cell { navajowhite } else { peru };

                let x = cells_size * (col as f64 + 0.5);
                let y = cells_size * (row as f64 + 0.5);

                cx.set_source_rgb(background_color.0, background_color.1, background_color.2);
                cx.rectangle(x, y, cells_size, cells_size);
                cx.fill().unwrap();
            }
        }
    }

    pub fn paint_pieces(
        cx: &Context,
        cells_size: f64,
        widget_board: &ChessBoard,
        logical_board: Board,
    ) {
        for row in 0..8 {
            for col in 0..8 {
                let file = col;
                let rank = 7 - row;
                let square_index = file + 8 * rank;
                let square = SQ::from(square_index);
                let piece = logical_board.piece_at_sq(square);

                if piece == Piece::None {
                    continue;
                }

                let x = cells_size as f64 * (file as f64 + 0.5);
                let y = cells_size as f64 * (7.5 - rank as f64);
                let piece_type = Painter::get_piece_type_from(piece);
                Painter::draw_piece(cx, widget_board, piece_type, x, y);
            }
        }
    }

    fn draw_piece(cx: &Context, board: &ChessBoard, piece_type: char, x: f64, y: f64) {
        let pixbuf = &board.model.pieces_images.pixbufs[&piece_type];
        cx.set_source_pixbuf(pixbuf, x, y);
        cx.paint().unwrap();
    }

    fn get_piece_type_from(piece: Piece) -> char {
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
}

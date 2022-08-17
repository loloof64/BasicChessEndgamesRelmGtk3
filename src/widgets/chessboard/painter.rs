use super::utils::get_piece_type_from;
use super::{ChessBoard, DragAndDropData};

use core::ascii;
use gtk::{cairo::Context, prelude::*};
use pleco::{Board, Piece, SQ};
use std::f64::consts::PI;

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
        reversed: bool,
    ) {
        for row in 0..8 {
            for col in 0..8 {
                let file = if reversed { 7 - col } else { col };
                let rank = if reversed { row } else { 7 - row };
                let square_index = file + 8 * rank;
                let square = SQ::from(square_index);
                let piece = logical_board.piece_at_sq(square);

                let is_moved_piece = match widget_board.model.dnd_data {
                    Some(ref dnd_data) => {
                        file == dnd_data.start_file && rank == dnd_data.start_rank
                    },
                    None => false,
                };
                let empty_square = piece == Piece::None;

                if empty_square || is_moved_piece {
                    continue;
                }


                let x = cells_size as f64 * (col as f64 + 0.5);
                let y = cells_size as f64 * (row as f64 + 0.5);
                let piece_type = get_piece_type_from(piece);
                Painter::draw_piece(cx, widget_board, piece_type, x, y);
            }
        }
    }

    pub fn draw_coordinates(cx: &Context, cells_size: f64, reversed: bool) {
        cx.set_source_rgb(0.78, 0.78, 0.47);
        cx.set_font_size(cells_size * 0.3);
        for col in 0..8 {
            let file = if reversed { 7 - col } else { col };
            let file_letter = (ascii::escape_default(b'A').next().unwrap() + file) as char;
            let file_string = format!("{}", file_letter);

            let x = cells_size * (0.9 + col as f64);
            let y1 = cells_size * 0.35;
            let y2 = cells_size * 8.85;

            cx.move_to(x, y1);
            cx.show_text(&file_string).unwrap();

            cx.move_to(x, y2);
            cx.show_text(&file_string).unwrap();
        }

        for row in 0..8 {
            let rank = if reversed { row } else { 7 - row };
            let rank_letter = (ascii::escape_default(b'1').next().unwrap() + rank) as char;
            let rank_string = format!("{}", rank_letter);

            let y = cells_size * (1.15 + row as f64);
            let x1 = cells_size * 0.15;
            let x2 = cells_size * 8.65;

            cx.move_to(x1, y);
            cx.show_text(&rank_string).unwrap();

            cx.move_to(x2, y);
            cx.show_text(&rank_string).unwrap();
        }
    }

    pub fn draw_player_turn(cx: &Context, cells_size: f64, white_turn: bool) {
        let color = if white_turn {
            (1.0, 1.0, 1.0)
        } else {
            (0.0, 0.0, 0.0)
        };
        let location = cells_size as f64 * 8.75;
        let radius = cells_size as f64 * 0.25;

        cx.set_source_rgb(color.0, color.1, color.2);
        cx.arc(location, location, radius, 0.0, 2.0 * PI);
        cx.fill().unwrap();
    }

    pub fn draw_moved_piece(cx: &Context, board: &ChessBoard) {
        let drag_drop_data = board.model.dnd_data.as_ref().unwrap();
        let half_cells_size = board.common_size() as f64 * 0.055;
        Painter::draw_piece(
            cx,
            board,
            drag_drop_data.piece,
            drag_drop_data.x - half_cells_size,
            drag_drop_data.y - half_cells_size,
        );
    }

    fn draw_piece(cx: &Context, board: &ChessBoard, piece_type: char, x: f64, y: f64) {
        let pixbuf = &board.model.pieces_images.pixbufs[&piece_type];
        cx.set_source_pixbuf(pixbuf, x, y);
        cx.paint().unwrap();
    }
}

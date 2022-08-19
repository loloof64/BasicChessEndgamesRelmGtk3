use super::ChessBoard;
use super::{pieces_images::PiecesImages, utils::get_piece_type_from};

use core::ascii;
use gtk::{cairo::Context, prelude::*};
use owlchess::{File, Rank};
use std::f64::consts::PI;

pub struct Painter;

impl Painter {
    pub const BUTTON_Y1_RATIO: f64 = 0.05;
    pub const BUTTON_Y2_RATIO: f64 = 0.80;
    pub const BUTTON_SIZE_RATIO: f64 = 0.15;

    pub const QUEEN_BUTTON_X_RATIO: f64 = 0.05;
    pub const ROOK_BUTTON_X_RATIO: f64 = 0.30;
    pub const BISHOP_BUTTON_X_RATIO: f64 = 0.55;
    pub const KNIGHT_BUTTON_X_RATIO: f64 = 0.80;

    pub fn draw(board: &mut ChessBoard) -> anyhow::Result<()> {
        let size = board.common_size();
        let cells_size = (size as f64) * 0.111;
        let fen_parts: Vec<String> = board
            .model
            .board
            .as_fen()
            .split(" ")
            .map(|e| String::from(e))
            .collect();
        let white_turn = fen_parts[1] == "w";
        let reversed = board.model.reversed;

        let image = gtk::cairo::ImageSurface::create(gtk::cairo::Format::ARgb32, size, size)?;
        let context = gtk::cairo::Context::new(&image)?;

        let drag_drop_data = board.model.dnd_data.as_ref();

        Painter::clear_background(&context, size as f64);
        Painter::paint_cells(&context, cells_size, board);
        Painter::draw_coordinates(&context, cells_size, reversed);
        Painter::paint_pieces(&context, cells_size, board, reversed);
        Painter::draw_player_turn(&context, cells_size, white_turn);

        if let Some(drag_drop_data) = drag_drop_data {
            Painter::draw_moved_piece(&context, board);
            if let Some(white_turn) = drag_drop_data.pending_promotion {
                Painter::draw_promotion_buttons(&context, board, white_turn);
            }
        }

        board.set_image(&image)?;
        Ok(())
    }

    fn clear_background(cx: &Context, size: f64) {
        cx.set_source_rgb(0.3, 0.3, 0.8);
        cx.rectangle(0.0, 0.0, size, size);
        cx.fill().unwrap();
    }

    fn paint_cells(cx: &Context, cells_size: f64, widget_board: &ChessBoard) {
        for row in 0..8 {
            for col in 0..8 {
                let file = if widget_board.model.reversed {
                    7 - col
                } else {
                    col
                };
                let rank = if widget_board.model.reversed {
                    row
                } else {
                    7 - row
                };

                let navajowhite = (1.0, 0.87, 0.68);
                let peru = (0.8, 0.52, 0.25);

                let olive = (0.5, 0.5, 0.0);
                let indian_red = (0.8, 0.36, 0.36);

                let is_white_cell = (row + col) % 2 == 0;
                let mut background_color = if is_white_cell { navajowhite } else { peru };

                let is_target_cell = match widget_board.model.dnd_data.as_ref() {
                    Some(drag_drop_data) => {
                        drag_drop_data.target_file == file && drag_drop_data.target_rank == rank
                    }
                    None => false,
                };

                let is_start_cell = match widget_board.model.dnd_data.as_ref() {
                    Some(drag_drop_data) => {
                        drag_drop_data.start_file == file && drag_drop_data.start_rank == rank
                    }
                    None => false,
                };

                if is_target_cell {
                    background_color = indian_red;
                }

                if is_start_cell {
                    background_color = olive;
                }

                let x = cells_size * (col as f64 + 0.5);
                let y = cells_size * (row as f64 + 0.5);

                cx.set_source_rgb(background_color.0, background_color.1, background_color.2);
                cx.rectangle(x, y, cells_size, cells_size);
                cx.fill().unwrap();
            }
        }
    }

    fn paint_pieces(cx: &Context, cells_size: f64, board: &ChessBoard, reversed: bool) {
        for row in 0..8 {
            for col in 0..8 {
                let file = if reversed { 7 - col } else { col } as u8;
                let rank = if reversed { row } else { 7 - row } as u8;
                let square = board.model.board.get2(
                    File::from_index(file as usize),
                    Rank::from_index((7 - rank) as usize),
                );
                let piece_type = square.piece();
                let piece_color = square.color();

                let is_moved_piece = match board.model.dnd_data {
                    Some(ref dnd_data) => {
                        file == dnd_data.start_file && rank == dnd_data.start_rank
                    }
                    None => false,
                };
                let empty_square = piece_type == None || piece_color == None;

                if empty_square || is_moved_piece {
                    continue;
                }

                let x = cells_size as f64 * (col as f64 + 0.5);
                let y = cells_size as f64 * (row as f64 + 0.5);
                let piece_type = get_piece_type_from(piece_type.unwrap(), piece_color.unwrap());
                Painter::draw_piece(cx, board, piece_type, x, y);
            }
        }
    }

    fn draw_coordinates(cx: &Context, cells_size: f64, reversed: bool) {
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

    fn draw_player_turn(cx: &Context, cells_size: f64, white_turn: bool) {
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

    fn draw_moved_piece(cx: &Context, board: &ChessBoard) {
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

    fn draw_promotion_buttons(cx: &Context, board: &ChessBoard, white_turn: bool) {
        let board_size = board.common_size() as f64;
        let reversed = board.model.reversed;
        let y = if white_turn {
            board_size
                * (if reversed {
                    Painter::BUTTON_Y1_RATIO
                } else {
                    Painter::BUTTON_Y2_RATIO
                })
        } else {
            board_size
                * (if reversed {
                    Painter::BUTTON_Y2_RATIO
                } else {
                    Painter::BUTTON_Y1_RATIO
                })
        };
        let x_queen = board_size * Painter::QUEEN_BUTTON_X_RATIO;
        let x_rook = board_size * Painter::ROOK_BUTTON_X_RATIO;
        let x_bishop = board_size * Painter::BISHOP_BUTTON_X_RATIO;
        let x_knight = board_size * Painter::KNIGHT_BUTTON_X_RATIO;

        let button_size = board_size * Painter::BUTTON_SIZE_RATIO;
        let button_color = (0.6, 0.5, 0.8, 0.5);
        cx.set_source_rgba(
            button_color.0,
            button_color.1,
            button_color.2,
            button_color.3,
        );

        cx.rectangle(x_queen, y, button_size, button_size);
        cx.fill().unwrap();
        cx.rectangle(x_rook, y, button_size, button_size);
        cx.fill().unwrap();
        cx.rectangle(x_bishop, y, button_size, button_size);
        cx.fill().unwrap();
        cx.rectangle(x_knight, y, button_size, button_size);
        cx.fill().unwrap();

        let button_size = button_size.floor() as i32;

        let queen_pixbuf =
            PiecesImages::get_piece_pixbuf(if white_turn { 'Q' } else { 'q' }, button_size)
                .unwrap();
        let rook_pixbuf =
            PiecesImages::get_piece_pixbuf(if white_turn { 'R' } else { 'r' }, button_size)
                .unwrap();
        let bishop_pixbuf =
            PiecesImages::get_piece_pixbuf(if white_turn { 'B' } else { 'b' }, button_size)
                .unwrap();
        let knight_pixbuf =
            PiecesImages::get_piece_pixbuf(if white_turn { 'N' } else { 'n' }, button_size)
                .unwrap();

        cx.set_source_pixbuf(&queen_pixbuf, x_queen, y);
        cx.paint().unwrap();
        cx.set_source_pixbuf(&rook_pixbuf, x_rook, y);
        cx.paint().unwrap();
        cx.set_source_pixbuf(&bishop_pixbuf, x_bishop, y);
        cx.paint().unwrap();
        cx.set_source_pixbuf(&knight_pixbuf, x_knight, y);
        cx.paint().unwrap();
    }
}

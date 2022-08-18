use gtk::gdk::{EventButton, EventMotion};
use pleco::{Piece, Player, SQ};

use super::{
    utils::{get_piece_type_from, get_uci_move_for, is_side_piece},
    ChessBoard, DragAndDropData,
};

pub struct MouseHandler;

impl MouseHandler {
    pub fn handle_button_down(board: &mut ChessBoard, event: EventButton) {
        let (x, y) = event.position();
        let cells_size = board.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if board.model.reversed { 7 - col } else { col };
        let rank = if board.model.reversed { row } else { 7 - row };

        let in_bounds = file >= 0 && file <= 7 && rank >= 0 && rank <= 7;
        if in_bounds {
            let square_index = file as u8 + 8 * rank as u8;
            let square = SQ::from(square_index);
            let piece = board.model.board.piece_at_sq(square);

            let not_empty_piece = piece != Piece::None;
            let white_turn = board.model.board.turn() == Player::White;
            let our_piece = is_side_piece(piece, white_turn);

            if not_empty_piece && our_piece {
                let piece = get_piece_type_from(piece);
                let drag_drop_data = DragAndDropData {
                    piece,
                    x,
                    y,
                    start_file: file as u8,
                    start_rank: rank as u8,
                    target_file: file as u8,
                    target_rank: rank as u8,
                };
                board.model.dnd_data = Some(drag_drop_data);
            }
        }
    }

    pub fn handle_button_up(board: &mut ChessBoard, event: EventButton) {
        let (x, y) = event.position();
        let cells_size = board.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if board.model.reversed { 7 - col } else { col };
        let rank = if board.model.reversed { row } else { 7 - row };

        if board.model.dnd_data.is_some() {
            let start_file = board.model.dnd_data.as_ref().unwrap().start_file;
            let start_rank = board.model.dnd_data.as_ref().unwrap().start_rank;

            let start_square_index = start_file + 8 * start_rank;
            let start_square = SQ::from(start_square_index);
            let target_square_index = (file + 8 * rank) as u8;
            let target_square = SQ::from(target_square_index);

            let uci_move = get_uci_move_for(start_square, target_square, None);
            board.model.board.apply_uci_move(&uci_move);
        }

        board.model.dnd_data = None;
    }

    pub fn handle_mouse_drag(board: &mut ChessBoard, event: EventMotion) {
        let (x, y) = event.position();
        let cells_size = board.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if board.model.reversed { 7 - col } else { col };
        let rank = if board.model.reversed { row } else { 7 - row };

        match board.model.dnd_data {
            Some(ref mut dnd_data) => {
                dnd_data.x = x;
                dnd_data.y = y;

                let in_bounds = file >= 0 && file <= 7 && rank >= 0 && rank <= 7;

                if in_bounds {
                    dnd_data.target_file = file as u8;
                    dnd_data.target_rank = rank as u8;
                }
            }
            _ => {}
        };
    }
}

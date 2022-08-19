use gtk::gdk::{EventButton, EventMotion};
use owlchess::{Color, File, Move, Rank};

use super::{
    painter::Painter,
    utils::{get_piece_type_from, get_uci_move_for},
    ChessBoard, DragAndDropData,
};

pub struct MouseHandler;

impl MouseHandler {
    pub fn handle_button_down(board: &mut ChessBoard, event: EventButton) {
        // Cancelling if there is a pending promotion move.
        let dnd_data = board.model.dnd_data.as_ref();
        match dnd_data {
            Some(dnd_data) => {
                if dnd_data.pending_promotion.is_some() {
                    MouseHandler::handle_promotion_button_click(board, event);
                    return;
                }
            }
            _ => {}
        }

        let (x, y) = event.position();
        let cells_size = board.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if board.model.reversed { 7 - col } else { col };
        let rank = if board.model.reversed { row } else { 7 - row };

        let in_bounds = file >= 0 && file <= 7 && rank >= 0 && rank <= 7;
        if in_bounds {
            let square = board.model.board.get2(
                File::from_index(file as usize),
                Rank::from_index((7-rank) as usize),
            );
            let piece_type = square.piece();
            let piece_color = square.color();

            let not_empty_piece = piece_type != None && piece_color != None;
            let fen_parts: Vec<String> = board
                .model
                .board
                .as_fen()
                .split(" ")
                .map(|e| String::from(e))
                .collect();
            let white_turn = fen_parts[1] == "w";
            let our_piece = match piece_color {
                Some(piece_color) => {
                    (piece_color == Color::White && white_turn)
                        || (piece_color == Color::Black && !white_turn)
                }
                _ => false,
            };

            if not_empty_piece && our_piece {
                let piece = get_piece_type_from(piece_type.unwrap(), piece_color.unwrap());
                let drag_drop_data = DragAndDropData {
                    piece,
                    x,
                    y,
                    start_file: file as u8,
                    start_rank: rank as u8,
                    target_file: file as u8,
                    target_rank: rank as u8,
                    pending_promotion: None,
                };
                board.model.dnd_data = Some(drag_drop_data);
            }
        }
    }

    pub fn handle_button_up(board: &mut ChessBoard, event: EventButton) {
        // Cancelling if there is a pending promotion move.
        let dnd_data = board.model.dnd_data.as_ref();
        match dnd_data {
            Some(dnd_data) => {
                if dnd_data.pending_promotion.is_some() {
                    return;
                }
            }
            _ => {}
        }

        let (x, y) = event.position();
        let cells_size = board.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if board.model.reversed { 7 - col } else { col };
        let rank = if board.model.reversed { row } else { 7 - row };

        if board.model.dnd_data.is_some() {
            let dnd_data = board.model.dnd_data.as_mut().unwrap();
            let start_file = dnd_data.start_file;
            let start_rank = dnd_data.start_rank;

            let is_promotion_move = ((dnd_data.piece == 'P' && rank == 7)
                || (dnd_data.piece == 'p' && rank == 0))
                && file >= 0
                && file <= 7;

            if is_promotion_move {
                let fen_parts: Vec<String> = board
                    .model
                    .board
                    .as_fen()
                    .split(" ")
                    .map(|e| String::from(e))
                    .collect();
                let white_turn = fen_parts[1] == "w";
                dnd_data.pending_promotion = Some(white_turn);
                dnd_data.target_file = file as u8;
                dnd_data.target_rank = rank as u8;
                return;
            }

            if file < 0 || file > 7 || rank < 0 || rank > 7 {
                board.model.dnd_data = None;
                return;
            }

            let uci_move = get_uci_move_for(start_file, start_rank, file as u8, rank as u8, None);
            let matching_move = Move::from_uci_legal(&uci_move, &board.model.board);

            if let Ok(matching_move) = matching_move {
                match board.model.board.make_move(matching_move) {
                    Ok(logical_board) => board.model.board = logical_board,
                    Err(_) => {}
                }
            }
        }

        board.model.dnd_data = None;
        board.handle_game_termination();
    }

    pub fn handle_mouse_drag(board: &mut ChessBoard, event: EventMotion) {
        // Cancelling if there is a pending promotion move.
        let dnd_data = board.model.dnd_data.as_ref();
        match dnd_data {
            Some(dnd_data) => {
                if dnd_data.pending_promotion.is_some() {
                    return;
                }
            }
            _ => {}
        }

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

    pub fn handle_promotion_button_click(board: &mut ChessBoard, event: EventButton) {
        let (x, y) = event.position();
        let board_size = board.common_size() as f64;
        let reversed = board.model.reversed;
        let white_turn = board
            .model
            .dnd_data
            .as_ref()
            .unwrap()
            .pending_promotion
            .unwrap();

        let buttons_y = if white_turn {
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

        let queen_button_x = board_size * Painter::QUEEN_BUTTON_X_RATIO;
        let rook_button_x = board_size * Painter::ROOK_BUTTON_X_RATIO;
        let bishop_button_x = board_size * Painter::BISHOP_BUTTON_X_RATIO;
        let knight_button_x = board_size * Painter::KNIGHT_BUTTON_X_RATIO;

        let buttons_size = board_size * Painter::BUTTON_SIZE_RATIO;

        let y_in_range = y >= buttons_y && y <= (buttons_y + buttons_size);
        let x_in_queen_button = x >= queen_button_x && x <= (queen_button_x + buttons_size);
        let x_in_rook_button = x >= rook_button_x && x <= (rook_button_x + buttons_size);
        let x_in_bishop_button = x >= bishop_button_x && x <= (bishop_button_x + buttons_size);
        let x_in_knight_button = x >= knight_button_x && x <= (knight_button_x + buttons_size);

        let queen_button_clicked = y_in_range && x_in_queen_button;
        let rook_button_clicked = y_in_range && x_in_rook_button;
        let bishop_button_clicked = y_in_range && x_in_bishop_button;
        let knight_button_clicked = y_in_range && x_in_knight_button;

        if queen_button_clicked {
            board.commit_promotion('q');
        } else if rook_button_clicked {
            board.commit_promotion('r');
        } else if bishop_button_clicked {
            board.commit_promotion('b');
        } else if knight_button_clicked {
            board.commit_promotion('n');
        }
    }
}

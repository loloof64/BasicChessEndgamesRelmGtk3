use gtk::gdk::{EventButton, EventMotion};
use gtk::prelude::*;
use owlchess::chain::BaseMoveChain;
use owlchess::{Board, MoveChain, Make, Outcome};
use relm::{Widget, StreamHandle};
use relm_derive::{widget, Msg};

mod mouse_handler;
mod painter;
mod pieces_images;
mod utils;

use anyhow::Context;

#[derive(Msg)]
#[allow(dead_code)]
pub enum Msg {
    Repaint,
    UpdatePiecesImagesSize,
    ToggleOrientation,
    SetReversed(bool),
    ButtonDown(EventButton),
    ButtonUp(EventButton),
    MouseMoved(EventMotion),
    GameOver(Outcome),
}

use self::mouse_handler::MouseHandler;
use self::utils::get_uci_move_for;
use self::Msg::*;

use super::mainwindow;

pub struct DragAndDropData {
    piece: char,
    x: f64,
    y: f64,
    start_file: u8,
    start_rank: u8,
    target_file: u8,
    target_rank: u8,
    pending_promotion: Option<bool>,
}

pub struct Model {
    #[allow(dead_code)]
    pieces_images: pieces_images::PiecesImages,
    board: Board,
    board_moves_chain: MoveChain,
    reversed: bool,
    dnd_data: Option<DragAndDropData>,
    window_stream: StreamHandle<mainwindow::Msg>,
    game_in_progress: bool,
}

#[widget]
impl Widget for ChessBoard {
    view! {
        gtk::EventBox {
            #[name="drawing_area"]
            gtk::DrawingArea {
                draw(_, cx) => (Repaint, gtk::Inhibit(false)),
            },
            button_press_event(_drawing_area, event) =>  (ButtonDown(event.clone()), gtk::Inhibit(false)),
            button_release_event(_drawing_area, event) =>  (ButtonUp(event.clone()), gtk::Inhibit(false)),
            motion_notify_event(_drawing_area, event) => (MouseMoved(event.clone()), gtk::Inhibit(false)),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Repaint => painter::Painter::draw(self).unwrap(),
            UpdatePiecesImagesSize => {
                let new_cells_size = (self.common_size() as f64 * 0.111) as i32;
                self.resize_pieces_images(new_cells_size)
                    .expect("Failed to resize pieces images.");
                painter::Painter::draw(self).unwrap();
            }
            ToggleOrientation => {
                self.model.reversed = !self.model.reversed;
                self.reverse_dragged_piece_position();
                painter::Painter::draw(self).unwrap();
            }
            SetReversed(reversed) => {
                let has_effect = reversed != self.model.reversed;
                if has_effect {
                    self.reverse_dragged_piece_position();
                }
                self.model.reversed = reversed;
                painter::Painter::draw(self).unwrap();
            }
            ButtonDown(event) => {
                MouseHandler::handle_button_down(self, event);
            }
            ButtonUp(event) => {
                MouseHandler::handle_button_up(self, event);
            }
            MouseMoved(event) => {
                MouseHandler::handle_mouse_drag(self, event);
            }
            GameOver(_) => {},
        }
    }

    fn model(window_stream: StreamHandle<mainwindow::Msg>) -> Model {
        let images = pieces_images::PiecesImages::new(30).expect("Failed to build pieces images.");
        let board = Board::initial();
        let board_clone = board.clone();
        Model {
            pieces_images: images,
            board,
            reversed: false,
            dnd_data: None,
            board_moves_chain: BaseMoveChain::new(board_clone),
            window_stream,
            game_in_progress: true,
        }
    }

    fn init_view(&mut self) {
        let size = 400;
        let cells_size = ((size as f64) * 0.111) as i32;
        self.widgets.drawing_area.set_size_request(400, 400);
        self.model.pieces_images =
            pieces_images::PiecesImages::new(cells_size).expect("Failed to build pieces images.");
    }
}

impl ChessBoard {
    pub fn start_new_game(&mut self) {
        let board = Board::initial();
        let board_clone = board.clone();
        self.model.board = board;
        self.model.board_moves_chain = MoveChain::new(board_clone);
        self.model.game_in_progress = true;
    }

    pub fn commit_promotion(&mut self, piece_type: char) {
        if piece_type != 'q' && piece_type != 'r' && piece_type != 'b' && piece_type != 'n' {
            return;
        }

        if self.model.dnd_data.is_none() {
            return;
        }

        let dnd_data = self.model.dnd_data.as_ref().unwrap();

        let start_file = dnd_data.start_file;
        let start_rank = dnd_data.start_rank;
        let target_file = dnd_data.target_file;
        let target_rank = dnd_data.target_rank;

        let uci_move = get_uci_move_for(
            start_file,
            start_rank,
            target_file,
            target_rank,
            Some(piece_type),
        );
        let matching_move = uci_move.into_move(&self.model.board);

        if let Ok(matching_move) = matching_move {
            match matching_move.make_raw(&mut self.model.board) {
                Ok(_) => self.model.board_moves_chain.push(matching_move).unwrap(),
                Err(_) => {}
            }
        }

        self.model.dnd_data = None;
    }

    fn set_image(&self, image: &gtk::cairo::ImageSurface) -> anyhow::Result<()> {
        let context = create_context(&self.widgets.drawing_area)?;

        context.set_source_surface(image, 0.0, 0.0)?;
        context.paint().expect("Failed to paint chess board.");

        Ok(())
    }

    fn common_size(&self) -> i32 {
        let width = self.widgets.drawing_area.allocated_width();
        let height = self.widgets.drawing_area.allocated_height();

        if width < height {
            width
        } else {
            height
        }
    }

    fn resize_pieces_images(&mut self, new_size: i32) -> anyhow::Result<()> {
        self.model.pieces_images = pieces_images::PiecesImages::new(new_size)?;

        Ok(())
    }

    fn reverse_dragged_piece_position(&mut self) {
        let this_size = { self.common_size() } as f64;
        let dnd_data = self.model.dnd_data.as_mut();
        match dnd_data {
            Some(dnd_data) => {
                let old_x = dnd_data.x;
                let old_y = dnd_data.y;

                let new_x = this_size as f64 - old_x;
                let new_y = this_size as f64 - old_y;

                dnd_data.x = new_x;
                dnd_data.y = new_y;
            }
            _ => {}
        };
    }
    fn handle_game_termination(&mut self, outcome: &Outcome) {
        self.model.game_in_progress = false;
        self.model.window_stream.emit(mainwindow::Msg::GameOver(*outcome));
    }
}

fn create_context(widget: &gtk::DrawingArea) -> anyhow::Result<gtk::cairo::Context> {
    let mut draw_handler = relm::DrawHandler::new().with_context(|| "draw handler")?;

    draw_handler.init(widget);

    let context = draw_handler.get_context().map(|x| x.clone())?;

    Ok(context)
}

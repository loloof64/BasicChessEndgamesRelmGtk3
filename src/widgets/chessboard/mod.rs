use gtk::gdk::{EventButton, EventMotion};
use gtk::prelude::*;
use pleco::{Board, SQ};
use relm::Widget;
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
}

use self::mouse_handler::MouseHandler;
use self::utils::get_uci_move_for;
use self::Msg::*;

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
    reversed: bool,
    dnd_data: Option<DragAndDropData>,
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
        }
    }

    fn model() -> Model {
        let images = pieces_images::PiecesImages::new(30).expect("Failed to build pieces images.");
        Model {
            pieces_images: images,
            board: Board::start_pos(),
            reversed: false,
            dnd_data: None,
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

        let start_square_index = start_file + 8 * start_rank;
        let start_square = SQ::from(start_square_index);
        let target_square_index = (target_file + 8 * target_rank) as u8;
        let target_square = SQ::from(target_square_index);

        let uci_move = get_uci_move_for(start_square, target_square, Some(piece_type));
        self.model.board.apply_uci_move(&uci_move);

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
}

fn create_context(widget: &gtk::DrawingArea) -> anyhow::Result<gtk::cairo::Context> {
    let mut draw_handler = relm::DrawHandler::new().with_context(|| "draw handler")?;

    draw_handler.init(widget);

    let context = draw_handler.get_context().map(|x| x.clone())?;

    Ok(context)
}

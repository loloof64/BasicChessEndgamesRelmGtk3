use gtk::gdk::EventButton;
use gtk::gdk::EventMotion;
use gtk::prelude::*;
use pleco::{Board, Piece, Player, SQ};
use relm::Widget;
use relm_derive::{widget, Msg};

mod painter;
mod pieces_images;
mod utils;

use utils::*;

use anyhow::{self, Context};

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

use self::Msg::*;

pub struct DragAndDropData {
    piece: char,
    x: f64,
    y: f64,
    start_file: u8,
    start_rank: u8,
    target_file: u8,
    target_rank: u8,
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
            Repaint => self.draw().unwrap(),
            UpdatePiecesImagesSize => {
                let new_cells_size = (self.common_size() as f64 * 0.111) as i32;
                self.resize_pieces_images(new_cells_size)
                    .expect("Failed to resize pieces images.");
                self.draw().unwrap();
            }
            ToggleOrientation => {
                self.model.reversed = !self.model.reversed;
                self.draw().unwrap();
            }
            SetReversed(reversed) => {
                self.model.reversed = reversed;
                self.draw().unwrap();
            }
            ButtonDown(event) => {
                self.handle_button_down(event);
            }
            ButtonUp(event) => {
                self.handle_button_up(event);
            }
            MouseMoved(event) => {
                self.handle_mouse_drag(event);
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
    fn set_image(&self, image: &gtk::cairo::ImageSurface) -> anyhow::Result<()> {
        let context = create_context(&self.widgets.drawing_area)?;

        context.set_source_surface(image, 0.0, 0.0)?;
        context.paint().expect("Failed to paint chess board.");

        Ok(())
    }

    fn draw(&self) -> anyhow::Result<()> {
        let size = self.common_size();
        let cells_size = (size as f64) * 0.111;
        let turn = self.model.board.turn() == Player::White;
        let reversed = self.model.reversed;

        let image = gtk::cairo::ImageSurface::create(gtk::cairo::Format::ARgb32, size, size)?;
        let context = gtk::cairo::Context::new(&image)?;

        let drag_drop_data = self.model.dnd_data.as_ref();

        painter::Painter::clear_background(&context, size as f64);
        painter::Painter::paint_cells(&context, cells_size, &self);
        painter::Painter::draw_coordinates(&context, cells_size, reversed);
        painter::Painter::paint_pieces(
            &context,
            cells_size,
            &self,
            self.model.board.clone(),
            reversed,
        );
        painter::Painter::draw_player_turn(&context, cells_size, turn);

        if drag_drop_data.is_some() {
            painter::Painter::draw_moved_piece(&context, self);
        }

        self.set_image(&image)?;
        Ok(())
    }

    fn handle_button_down(&mut self, event: EventButton) {
        let (x, y) = event.position();
        let cells_size = self.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if self.model.reversed { 7 - col } else { col };
        let rank = if self.model.reversed { row } else { 7 - row };

        let in_bounds = file >= 0 && file <= 7 && rank >= 0 && rank <= 7;
        if in_bounds {
            let square_index = file as u8 + 8 * rank as u8;
            let square = SQ::from(square_index);
            let piece = self.model.board.piece_at_sq(square);

            let not_empty_piece = piece != Piece::None;
            let white_turn = self.model.board.turn() == Player::White;
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
                self.model.dnd_data = Some(drag_drop_data);
            }
        }
    }

    fn handle_button_up(&mut self, event: EventButton) {
        let (x, y) = event.position();
        let cells_size = self.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if self.model.reversed { 7 - col } else { col };
        let rank = if self.model.reversed { row } else { 7 - row };

        if self.model.dnd_data.is_some() {
            let start_file = self.model.dnd_data.as_ref().unwrap().start_file;
            let start_rank = self.model.dnd_data.as_ref().unwrap().start_rank;

            let start_square_index = start_file + 8 * start_rank;
            let start_square = SQ::from(start_square_index);
            let target_square_index = (file + 8 * rank) as u8;
            let target_square = SQ::from(target_square_index);

            let uci_move = get_uci_move_for(start_square, target_square, None);
            self.model.board.apply_uci_move(&uci_move);
        }

        self.model.dnd_data = None;
    }

    fn handle_mouse_drag(&mut self, event: EventMotion) {
        let (x, y) = event.position();
        let cells_size = self.common_size() as f64 * 0.111;
        let col = ((x - cells_size * 0.5) / cells_size).floor() as i16;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i16;
        let file = if self.model.reversed { 7 - col } else { col };
        let rank = if self.model.reversed { row } else { 7 - row };

        match self.model.dnd_data {
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
}

fn create_context(widget: &gtk::DrawingArea) -> anyhow::Result<gtk::cairo::Context> {
    let mut draw_handler = relm::DrawHandler::new().with_context(|| "draw handler")?;

    draw_handler.init(widget);

    let context = draw_handler.get_context().map(|x| x.clone())?;

    Ok(context)
}

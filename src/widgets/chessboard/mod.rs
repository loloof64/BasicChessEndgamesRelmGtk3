use gtk::prelude::*;
use gtk::{gdk::EventButton, prelude::*};
use pleco::{Board, Player};
use relm::Widget;
use relm_derive::{widget, Msg};

mod painter;
mod pieces_images;

use anyhow::{self, Context};

#[derive(Msg)]
pub enum Msg {
    Repaint,
    UpdatePiecesImagesSize,
    ToggleOrientation,
    SetReversed(bool),
    ButtonDown(EventButton),
    ButtonUp(EventButton),
}

use self::Msg::*;

struct DragAndDropData {
    piece: char,
    x: f64,
    y: f64,
    startFile: u32,
    startRank: u32,
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
                println!(
                    "Drag start at ({}, {}).",
                    event.position().0,
                    event.position().1
                );
            }
            ButtonUp(event) => {
                println!(
                    "Drag end at ({}, {}).",
                    event.position().0,
                    event.position().1
                )
            }
        }
    }

    fn model() -> Model {
        let images = pieces_images::PiecesImages::new(30).expect("Failed to build pieces images.");
        Model {
            pieces_images: images,
            board: Board::from_fen(
                "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
            )
            .unwrap(),
            reversed: true,
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

        painter::Painter::clear_background(&context, size as f64);
        painter::Painter::paint_cells(&context, cells_size);
        painter::Painter::draw_coordinates(&context, cells_size, reversed);
        painter::Painter::paint_pieces(
            &context,
            cells_size,
            self,
            self.model.board.clone(),
            reversed,
        );
        painter::Painter::draw_player_turn(&context, cells_size, turn);

        self.set_image(&image)?;
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
}

fn create_context(widget: &gtk::DrawingArea) -> anyhow::Result<gtk::cairo::Context> {
    let mut draw_handler = relm::DrawHandler::new().with_context(|| "draw handler")?;

    draw_handler.init(widget);

    let context = draw_handler.get_context().map(|x| x.clone())?;

    Ok(context)
}

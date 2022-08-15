use gtk::prelude::WidgetExt;
use pleco::Board;
use relm::Widget;
use relm_derive::{widget, Msg};

mod painter;
mod pieces_images;

#[derive(Msg)]
pub enum Msg {
    Repaint,
    UpdatePiecesImagesSize,
}

use self::Msg::*;

pub struct Model {
    #[allow(dead_code)]
    pieces_images: pieces_images::PiecesImages,
    board: Board,
}

#[widget]
impl Widget for ChessBoard {
    view! {
        #[name="drawing_area"]
        gtk::DrawingArea {
            draw(_, cx) => (Repaint, gtk::Inhibit(false)),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Repaint => self.draw().unwrap(),
            UpdatePiecesImagesSize => {
                let new_cells_size = (self.common_size() as f64 * 0.111) as i32;
                self.resize_pieces_images(new_cells_size);
                self.draw().unwrap();
            }
        }
    }

    fn model() -> Model {
        let images = pieces_images::PiecesImages::new(30);
        Model {
            pieces_images: images,
            board: Board::start_pos(),
        }
    }

    fn init_view(&mut self) {
        let size = 400;
        let cells_size = ((size as f64) * 0.111) as i32;
        self.widgets.drawing_area.set_size_request(400, 400);
        self.model.pieces_images = pieces_images::PiecesImages::new(cells_size);
    }
}

impl ChessBoard {
    fn set_image(&self, image: &gtk::cairo::ImageSurface) -> Result<(), gtk::cairo::Error> {
        let context = create_context(&self.widgets.drawing_area)?;

        context.set_source_surface(image, 0.0, 0.0)?;
        context.paint()
    }

    fn draw(&self) -> Result<(), gtk::cairo::Error> {
        let size = self.common_size();
        let cells_size = (size as f64) * 0.111;

        let image = gtk::cairo::ImageSurface::create(gtk::cairo::Format::ARgb32, size, size)?;
        let context = gtk::cairo::Context::new(&image)?;

        painter::Painter::clear_background(&context, size as f64);
        painter::Painter::paint_cells(&context, cells_size);
        painter::Painter::paint_pieces(&context, cells_size, self, self.model.board.clone());

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

    fn resize_pieces_images(&mut self, new_size: i32) {
        self.model.pieces_images = pieces_images::PiecesImages::new(new_size);
    }
}

fn create_context(widget: &gtk::DrawingArea) -> Result<gtk::cairo::Context, gtk::cairo::Error> {
    let mut draw_handler = relm::DrawHandler::new().expect("draw handler");

    draw_handler.init(widget);

    draw_handler.get_context().map(|x| x.clone())
}

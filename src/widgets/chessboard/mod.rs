use relm::Widget;
use relm_derive::{widget, Msg};
use gtk::prelude::WidgetExt;

mod pieces_images;
mod painter;

#[derive(Msg)]
pub enum Msg {
    Repaint,
}

use self::Msg::*;

pub struct Model {
    pieces_images: pieces_images::PiecesImages,
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
        }
    }

    fn model() -> Model {
        let images = pieces_images::PiecesImages::new(30);
        Model {
            pieces_images: images,
        }
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

        let image =
            gtk::cairo::ImageSurface::create(gtk::cairo::Format::ARgb32, size, size)?;
        let context = gtk::cairo::Context::new(&image)?;

        painter::Painter::clear_background(&context, size as f64);
        painter::Painter::draw_piece(&context, self, 'B', 30.0, 60.0)?;
        painter::Painter::draw_piece(&context, self, 'n', 80.0, 120.0)?;

        self.set_image(&image)?;
        Ok(())
    }

    fn common_size(&self) -> i32 {
        let width = self.widgets.drawing_area.allocated_width();
        let height = self.widgets.drawing_area.allocated_height();

        if width < height {width} else {height}
    }
}

fn create_context(widget: &gtk::DrawingArea) -> Result<gtk::cairo::Context, gtk::cairo::Error> {
    let mut draw_handler = relm::DrawHandler::new().expect("draw handler");

    draw_handler.init(widget);

    draw_handler.get_context().map(|x| x.clone())
}
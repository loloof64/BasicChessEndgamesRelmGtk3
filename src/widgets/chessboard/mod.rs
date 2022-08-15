use relm::Widget;
use relm_derive::{widget, Msg};
use gtk::prelude::WidgetExt;
use gtk::cairo::Context;

#[derive(Msg)]
pub enum Msg {
    Repaint,
}

use self::Msg::*;

pub struct Model {}

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
        Model {}
    }
}

impl ChessBoard {
    fn set_image(&self, image: &gtk::cairo::ImageSurface) -> Result<(), gtk::cairo::Error> {
        let context = create_context(&self.widgets.drawing_area)?;

        context.set_source_surface(image, 0.0, 0.0)?;
        context.paint()
    }

    fn draw(&self) -> Result<(), gtk::cairo::Error> {
        let width = self.widgets.drawing_area.allocated_width();
        let height = self.widgets.drawing_area.allocated_height();

        let image =
            gtk::cairo::ImageSurface::create(gtk::cairo::Format::ARgb32, width, height).unwrap();
        let context = gtk::cairo::Context::new(&image)?;

        self.clear_background(&context, width as f64, height as f64);
        self.set_image(&image)?;

        Ok(())
    }

    fn clear_background(&self, cx: &Context,  width: f64, height: f64) {
        cx.set_source_rgb(0.3, 0.3, 0.8);
        cx.rectangle(0.0, 0.0, width, height);
        cx.fill().unwrap();
    }
}

fn create_context(widget: &gtk::DrawingArea) -> Result<gtk::cairo::Context, gtk::cairo::Error> {
    let mut draw_handler = relm::DrawHandler::new().expect("draw handler");

    draw_handler.init(widget);

    draw_handler.get_context().map(|x| x.clone())
}
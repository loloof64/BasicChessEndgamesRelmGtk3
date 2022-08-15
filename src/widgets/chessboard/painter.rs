use super::ChessBoard;
use gtk::{cairo::Context, prelude::GdkContextExt};

pub struct Painter {}

impl Painter {
    pub fn clear_background(cx: &Context, size: f64) {
        cx.set_source_rgb(0.3, 0.3, 0.8);
        cx.rectangle(0.0, 0.0, size, size);
        cx.fill().unwrap();
    }

    pub fn draw_piece(
        cx: &Context,
        board: &ChessBoard,
        piece_type: char,
        x: f64,
        y: f64,
    ) -> Result<(), gtk::cairo::Error> {
        let pixbuf = &board.model.pieces_images.pixbufs[&piece_type];
        cx.set_source_pixbuf(pixbuf, x, y);
        cx.paint()?;

        Ok(())
    }
}

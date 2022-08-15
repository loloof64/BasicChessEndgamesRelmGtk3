use relm::Widget;
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {}

pub struct Model {}

#[widget]
impl Widget for ChessBoard {
    view! {
        gtk::DrawingArea {

        }
    }

    fn update(&mut self, event: Msg) {}

    fn model() -> Model {
        Model {}
    }
}

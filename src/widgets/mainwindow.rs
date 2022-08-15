use gtk::prelude::{WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use super::chessboard::ChessBoard;

#[widget]
impl Widget for MainWindow {
    view! {
        gtk::Window {
            #[name="board"]
            ChessBoard {

            },
            delete_event(_,_) => (Quit, gtk::Inhibit(false))
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    fn model() -> Model {
        Model {}
    }

    fn init_view(&mut self) {
        self.widgets.board.set_size_request(400, 400);
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

use self::Msg::*;

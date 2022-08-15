use gtk::prelude::WidgetExt;
use relm::Widget;
use relm_derive::{widget, Msg};

use super::chessboard::{ChessBoard, Msg as BoardMsg};

#[widget]
impl Widget for MainWindow {
    view! {
        #[name="root"]
        gtk::Window {
            #[name="board"]
            ChessBoard {
                halign: gtk::Align::Center,
                valign: gtk::Align::Center,
            },
            delete_event(_window, _event) => (Quit, gtk::Inhibit(false)),
            size_allocate(_window, _allocation) => board@BoardMsg::UpdatePiecesImagesSize,
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
        self.widgets.board.set_size_request(800, 800);
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

use self::Msg::*;

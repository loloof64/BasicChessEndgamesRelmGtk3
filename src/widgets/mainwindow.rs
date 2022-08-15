use gtk::prelude::WidgetExt;
use relm::Widget;
use relm_derive::{widget, Msg};

use super::chessboard::ChessBoard;

#[widget]
impl Widget for MainWindow {
    view! {
        #[name="root"]
        gtk::Window {
            #[name="board"]
            ChessBoard {

            },
            delete_event(_window, _event) => (Quit, gtk::Inhibit(false)),
            size_allocate(_window, _allocation) => Resize,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            Resize => self.update_on_resize(),
        }
    }

    fn model() -> Model {
        Model {}
    }

    fn init_view(&mut self) {
        self.widgets.board.set_size_request(400, 400);
    }
}

impl MainWindow {
    fn update_on_resize(&mut self) {
        self.widgets.root.queue_draw();
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Resize,
}

pub struct Model {}

use self::Msg::*;

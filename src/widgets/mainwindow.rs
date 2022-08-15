use relm::Widget;
use relm_derive::{widget, Msg};
use gtk::prelude::ButtonExt;

#[widget]
impl Widget for MainWindow {
    view! {
        gtk::Window {
            gtk::Button {
                clicked => Quit,
                label: "Click me",
            }
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit()
        }
    }

    fn model() -> Model {
        Model {

        }
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {

}

use self::Msg::*;
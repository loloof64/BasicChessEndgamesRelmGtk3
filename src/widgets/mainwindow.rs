use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use gtk::{
    prelude::{BoxExt, OrientableExt, ToolButtonExt, WidgetExt},
    traits::ToolbarExt,
    ToolButton,
};
use relm::{connect, Widget};
use relm_derive::{widget, Msg};

use super::chessboard::{ChessBoard, Msg as BoardMsg};

#[widget]
impl Widget for MainWindow {
    view! {
        #[name="root"]
        gtk::Window {
            gtk::Box {
                #[name="toolbar"]
                gtk::Toolbar {
                    style: gtk::ToolbarStyle::Icons,
                },
                #[name="board"]
                ChessBoard {
                    halign: gtk::Align::Center,
                    valign: gtk::Align::Center,
                },
                orientation: gtk::Orientation::Vertical,
                spacing: 5,
            },
            delete_event(_window, _event) => (Quit, gtk::Inhibit(false)),
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

        let reverse_pixbuf =
            get_image_pixbuf_from(include_bytes!("../assets/images/reverse.svg"), 30);
        let reverse_image = gtk::Image::from_pixbuf(Some(&reverse_pixbuf));
        let reverse_board_button = ToolButton::new(Some(&reverse_image), None);
        connect!(
            reverse_board_button,
            connect_clicked(_),
            self.components.board,
            BoardMsg::ToggleOrientation
        );

        self.widgets.toolbar.insert(&reverse_board_button, 0);

        self.widgets.root.show_all();
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

use self::Msg::*;

fn get_image_pixbuf_from(data: &[u8], size: i32) -> Pixbuf {
    let image_data = Bytes::from(data);
    let image_stream = MemoryInputStream::from_bytes(&image_data);

    Pixbuf::from_stream_at_scale(
        &image_stream,
        size,
        size,
        true,
        None::<&gtk::gio::Cancellable>,
    )
    .expect("Failed to interpret image.")
}

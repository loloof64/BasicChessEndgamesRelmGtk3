use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use gtk::{
    prelude::*, traits::ToolbarExt, ButtonsType, DialogFlags, MessageDialog, MessageType,
    ToolButton,
};
use owlchess::{Color, DrawReason, Outcome, WinReason};
use relm::{connect, Widget};
use relm_derive::{widget, Msg};

use super::chessboard::{ChessBoard, Msg as BoardMsg};
use BoardMsg::GameOver as BoardGameOver;

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
                    BoardGameOver(outcome) => GameOver(outcome),
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
            GameOver(outcome) => self.handle_game_termination(outcome),
        }
    }

    fn model() -> Model {
        Model {}
    }

    fn init_view(&mut self) {
        self.widgets.board.set_size_request(400, 400);

        let reverse_pixbuf =
            get_image_pixbuf_from(include_bytes!("../assets/images/reverse.svg"), 30)
                .expect("Failed to build image for reverse button.");
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

impl MainWindow {
    fn handle_game_termination(&self, outcome: Outcome) {
        let message = match outcome {
            Outcome::Draw(draw_type) => String::from(match draw_type {
                DrawReason::InsufficientMaterial => "Draw by insufficient material.",
                DrawReason::Stalemate => "Draw by stalemate.",
                DrawReason::Moves50 => "Draw by the 50 moves rule.",
                DrawReason::Moves75 => "Draw by the 75 moves rule.",
                DrawReason::Repeat3 => "Draw by three folds repetition.",
                DrawReason::Repeat5 => "Draw by five folds repetition.",
                _ => "Draw by unknown reason.",
            }),
            Outcome::Win { side, reason } => {
                let side_text = if side == Color::White {
                    "White"
                } else {
                    "Black"
                };
                match reason {
                    WinReason::Checkmate => {
                        format!("{} has won by checkmate.", side_text)
                    }
                    _ => format!("{} has won by unknown reason.", side_text),
                }
            }
        };
        let dialog = MessageDialog::new(
            Some(&self.widgets.root),
            DialogFlags::MODAL,
            MessageType::Info,
            ButtonsType::Ok,
            &message,
        );
        dialog.run();
        dialog.emit_close();
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    GameOver(Outcome),
}

pub struct Model {}

use self::Msg::*;

use anyhow::{self, Context};

fn get_image_pixbuf_from(data: &[u8], size: i32) -> anyhow::Result<Pixbuf> {
    let image_data = Bytes::from(data);
    let image_stream = MemoryInputStream::from_bytes(&image_data);

    let pixbuf = Pixbuf::from_stream_at_scale(
        &image_stream,
        size,
        size,
        true,
        None::<&gtk::gio::Cancellable>,
    )
    .with_context(|| "Failed to interpret image.")?;

    Ok(pixbuf)
}

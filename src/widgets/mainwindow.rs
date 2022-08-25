use gtk::ResponseType;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use gtk::{
    prelude::*, traits::ToolbarExt, ButtonsType, DialogFlags, MessageDialog, MessageType,
    ToolButton,
};
use owlchess::{Color, DrawReason, Outcome, WinReason};
use relm::{connect, Widget, Relm};
use relm_derive::{widget, Msg};

use super::chessboard::{ChessBoard, Msg as BoardMsg};
use BoardMsg::{GameOver as BoardGameOver, GameStarted as BoardGameStarted, GameStopped as BoardGameStopped};

use tr::tr;

#[widget]
impl Widget for MainWindow {
    view! {
        #[name="root"]
        gtk::Window {
            title: "Basic chess endgames",
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
                    BoardGameStarted => GameStarted,
                    BoardGameStopped => GameStoppedByUser,
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
            StopGame => self.show_stop_confirmation_dialog(),
            GameStarted => self.model.game_in_progress = true,
            GameStoppedByUser => self.handle_game_stopped_by_user(),
        }
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            game_in_progress: true, //TODO change to false later
        }
    }

    fn init_view(&mut self) {
        self.widgets.board.set_size_request(400, 400);

        let reverse_pixbuf =
            get_image_pixbuf_from(include_bytes!("../assets/images/reverse.svg"), 30)
                .expect("Failed to build image for reverse button.");
        let reverse_image = gtk::Image::from_pixbuf(Some(&reverse_pixbuf));
        let reverse_board_button = ToolButton::new(Some(&reverse_image), None);

        let stop_pixbuf = get_image_pixbuf_from(include_bytes!("../assets/images/stop.svg"), 30)
            .expect("Failed to build image for stop button.");
        let stop_image = gtk::Image::from_pixbuf(Some(&stop_pixbuf));
        let stop_button = ToolButton::new(Some(&stop_image), None);

        connect!(
            reverse_board_button,
            connect_clicked(_),
            self.components.board,
            BoardMsg::ToggleOrientation
        );

        connect!(
            stop_button,
            connect_clicked(_),
            self.model.relm,
            StopGame
        );

        self.widgets.toolbar.insert(&reverse_board_button, -1);
        self.widgets.toolbar.insert(&stop_button, -1);

        self.widgets.root.show_all();
    }
}

impl MainWindow {
    fn handle_game_termination(&mut self, outcome: Outcome) {
        let message = match outcome {
            Outcome::Draw(draw_type) => match draw_type {
                DrawReason::InsufficientMaterial => tr!("Draw by missing material."),
                DrawReason::Stalemate => tr!("Draw by stalemate."),
                DrawReason::Moves50 => tr!("Draw by the 50 moves rule."),
                DrawReason::Moves75 => tr!("Draw by the 75 moves rule."),
                DrawReason::Repeat3 => tr!("Draw by three fold repetition."),
                DrawReason::Repeat5 => tr!("Draw by five fold repetition."),
                _ => tr!("Draw by unknown reason."),
            },
            Outcome::Win { side, reason } => {
                let side_text = if side == Color::White {
                    tr!("White")
                } else {
                    tr!("Black")
                };
                match reason {
                    WinReason::Checkmate => {
                        tr!("{} won by checkmate.", side_text)
                    }
                    _ => tr!("{} won by unknown reason.", side_text),
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
        self.model.game_in_progress = false;
    }

    fn show_stop_confirmation_dialog(&self) {
        if ! self.model.game_in_progress {
            return ;
        }
        let message = tr!("Do you want to stop current game ?");
        let dialog = MessageDialog::new(
            Some(&self.widgets.root),
            DialogFlags::MODAL,
            MessageType::Question,
            ButtonsType::YesNo,
            &message,
        );
        let response = dialog.run();
        dialog.emit_close();

        if response == ResponseType::Yes {
            self.components.board.emit(BoardMsg::StopGame);
        }
    }

    fn handle_game_stopped_by_user(&mut self) {
        self.model.game_in_progress = false;

        let message = tr!("Game interrupted.");
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
    StopGame,
    GameStarted,
    GameStoppedByUser,
}

pub struct Model {
    relm: Relm<MainWindow>,
    game_in_progress: bool,
}

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

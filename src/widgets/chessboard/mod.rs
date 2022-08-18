use gtk::Box as GtkBox;
use gtk::Button;
use gtk::Image;
use gtk::gdk::EventButton;
use gtk::gdk::EventMotion;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use gtk::prelude::*;
use gtk::Dialog;
use pleco::Board;
use relm::Widget;
use relm_derive::{widget, Msg};

mod mouse_handler;
mod painter;
mod pieces_images;
mod utils;

use anyhow::{anyhow, Context};

#[derive(Msg)]
#[allow(dead_code)]
pub enum Msg {
    Repaint,
    UpdatePiecesImagesSize,
    ToggleOrientation,
    SetReversed(bool),
    ButtonDown(EventButton),
    ButtonUp(EventButton),
    MouseMoved(EventMotion),
}

use self::mouse_handler::MouseHandler;
use self::Msg::*;

pub struct DragAndDropData {
    piece: char,
    x: f64,
    y: f64,
    start_file: u8,
    start_rank: u8,
    target_file: u8,
    target_rank: u8,
    pending_promotion: bool,
}

pub struct Model {
    #[allow(dead_code)]
    pieces_images: pieces_images::PiecesImages,
    board: Board,
    reversed: bool,
    dnd_data: Option<DragAndDropData>,
}

#[widget]
impl Widget for ChessBoard {
    view! {
        gtk::EventBox {
            #[name="drawing_area"]
            gtk::DrawingArea {
                draw(_, cx) => (Repaint, gtk::Inhibit(false)),
            },
            button_press_event(_drawing_area, event) =>  (ButtonDown(event.clone()), gtk::Inhibit(false)),
            button_release_event(_drawing_area, event) =>  (ButtonUp(event.clone()), gtk::Inhibit(false)),
            motion_notify_event(_drawing_area, event) => (MouseMoved(event.clone()), gtk::Inhibit(false)),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Repaint => painter::Painter::draw(self).unwrap(),
            UpdatePiecesImagesSize => {
                let new_cells_size = (self.common_size() as f64 * 0.111) as i32;
                self.resize_pieces_images(new_cells_size)
                    .expect("Failed to resize pieces images.");
                painter::Painter::draw(self).unwrap();
            }
            ToggleOrientation => {
                self.model.reversed = !self.model.reversed;
                self.reverse_dragged_piece_position();
                painter::Painter::draw(self).unwrap();
            }
            SetReversed(reversed) => {
                let has_effect = reversed != self.model.reversed;
                if has_effect {
                    self.reverse_dragged_piece_position();
                }
                self.model.reversed = reversed;
                painter::Painter::draw(self).unwrap();
            }
            ButtonDown(event) => {
                MouseHandler::handle_button_down(self, event);
            }
            ButtonUp(event) => {
                MouseHandler::handle_button_up(self, event);
            }
            MouseMoved(event) => {
                MouseHandler::handle_mouse_drag(self, event);
            }
        }
    }

    fn model() -> Model {
        let images = pieces_images::PiecesImages::new(30).expect("Failed to build pieces images.");
        Model {
            pieces_images: images,
            board: Board::start_pos(),
            reversed: false,
            dnd_data: None,
        }
    }

    fn init_view(&mut self) {
        let size = 400;
        let cells_size = ((size as f64) * 0.111) as i32;
        self.widgets.drawing_area.set_size_request(400, 400);
        self.model.pieces_images =
            pieces_images::PiecesImages::new(cells_size).expect("Failed to build pieces images.");
    }
}

impl ChessBoard {
    pub fn show_promotion_dialog(&self, white_turn: bool) {
        let dialog = Dialog::new();

        dialog.set_attached_to(Some(&self.widgets.drawing_area));

        let dialog_area = dialog.content_area();

        let hbox = GtkBox::new(gtk::Orientation::Horizontal, 10);

        let size = (self.common_size() as f64 * 0.222).floor() as i32;

        let queen_pixbuf = ChessBoard::get_piece_pixbuf(if white_turn { 'Q' } else { 'q' }, size).unwrap();
        let rook_pixbuf = ChessBoard::get_piece_pixbuf(if white_turn { 'R' } else { 'r' }, size).unwrap();
        let bishop_pixbuf = ChessBoard::get_piece_pixbuf(if white_turn { 'B' } else { 'b' }, size).unwrap();
        let knight_pixbuf = ChessBoard::get_piece_pixbuf(if white_turn { 'N' } else { 'n' }, size).unwrap();

        let queen_button = Button::new();
        queen_button.set_image(Some(&Image::from_pixbuf(Some(&queen_pixbuf))));
        queen_button.connect_clicked(|_button| {
            println!("queen");
        });

        let rook_button = Button::new();
        rook_button.set_image(Some(&Image::from_pixbuf(Some(&rook_pixbuf))));
        rook_button.connect_clicked(|_button| {
            println!("rook");
        });

        let bishop_button = Button::new();
        bishop_button.set_image(Some(&Image::from_pixbuf(Some(&bishop_pixbuf))));
        bishop_button.connect_clicked(|_button| {
            println!("bishop");
        });

        let knight_button = Button::new();
        knight_button.set_image(Some(&Image::from_pixbuf(Some(&knight_pixbuf))));
        knight_button.connect_clicked(|_button| {
            println!("knight");
        });

        hbox.pack_start(&queen_button, true, false, 10);
        hbox.pack_start(&rook_button, true, false, 10);
        hbox.pack_start(&bishop_button, true, false, 10);
        hbox.pack_start(&knight_button, true, false, 10);

        dialog_area.pack_start(&hbox, true, true, 10);

        dialog.set_deletable(false);

        dialog.show_all();
    }

    fn get_piece_pixbuf(piece_type: char, size: i32) -> anyhow::Result<Pixbuf> {
        let piece_type_lowercase = piece_type.to_ascii_lowercase();
        if piece_type_lowercase == 'q'
            || piece_type_lowercase == 'r'
            || piece_type_lowercase == 'b'
            || piece_type_lowercase == 'n'
        {
            let data: &[u8] = match piece_type {
                'Q' => include_bytes!("./vectors/Chess_qlt45.svg"),
                'R' => include_bytes!("./vectors/Chess_rlt45.svg"),
                'B' => include_bytes!("./vectors/Chess_blt45.svg"),
                'N' => include_bytes!("./vectors/Chess_nlt45.svg"),
                'q' => include_bytes!("./vectors/Chess_qdt45.svg"),
                'r' => include_bytes!("./vectors/Chess_rdt45.svg"),
                'b' => include_bytes!("./vectors/Chess_bdt45.svg"),
                'n' => include_bytes!("./vectors/Chess_ndt45.svg"),
                _ => panic!("Forbidden piece type {}.", piece_type),
            };
            let data = data;
            let data = Bytes::from(data);
            let image_stream = MemoryInputStream::from_bytes(&data);

            let pixbuf = Pixbuf::from_stream_at_scale(
                &image_stream,
                size,
                size,
                true,
                None::<&gtk::gio::Cancellable>,
            )
            .with_context(|| "Failed to interpret image.")?;

            Ok(pixbuf)
        } else {
            Err(anyhow!("Forbidden piece type {}", piece_type))
        }
    }

    fn set_image(&self, image: &gtk::cairo::ImageSurface) -> anyhow::Result<()> {
        let context = create_context(&self.widgets.drawing_area)?;

        context.set_source_surface(image, 0.0, 0.0)?;
        context.paint().expect("Failed to paint chess board.");

        Ok(())
    }

    fn common_size(&self) -> i32 {
        let width = self.widgets.drawing_area.allocated_width();
        let height = self.widgets.drawing_area.allocated_height();

        if width < height {
            width
        } else {
            height
        }
    }

    fn resize_pieces_images(&mut self, new_size: i32) -> anyhow::Result<()> {
        self.model.pieces_images = pieces_images::PiecesImages::new(new_size)?;

        Ok(())
    }

    fn reverse_dragged_piece_position(&mut self) {
        let this_size = { self.common_size() } as f64;
        let dnd_data = self.model.dnd_data.as_mut();
        match dnd_data {
            Some(dnd_data) => {
                let old_x = dnd_data.x;
                let old_y = dnd_data.y;

                let new_x = this_size as f64 - old_x;
                let new_y = this_size as f64 - old_y;

                dnd_data.x = new_x;
                dnd_data.y = new_y;
            }
            _ => {}
        };
    }
}

fn create_context(widget: &gtk::DrawingArea) -> anyhow::Result<gtk::cairo::Context> {
    let mut draw_handler = relm::DrawHandler::new().with_context(|| "draw handler")?;

    draw_handler.init(widget);

    let context = draw_handler.get_context().map(|x| x.clone())?;

    Ok(context)
}

use gtk::{
    traits::{CssProviderExt, StyleContextExt, WidgetExt, FlowBoxExt},
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

mod utils;

#[widget]
impl Widget for History {
    view! {
        #[name="root"]
        gtk::FlowBox {
            widget_name: "history_root",
            halign: gtk::Align::Start,
            valign: gtk::Align::Start,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddMoveSan(san, white_player) => self.add_move_san(utils::san_to_fan(san, white_player), white_player),
        }
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            game_in_progress: false,
            move_number: 1,
        }
    }

    fn init_view(&mut self) {
        let style_context = self.widgets.root.style_context();
        let style = include_bytes!("./style.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
        
        //TODO only on new game
        self.add_move_number();
        self.widgets.root.show_all();
    }
}

impl History {
    fn add_move_san(&mut self, san: String, white_player: bool) {
        let button = gtk::Button::with_label(&san);
        let style_context = button.style_context();
        style_context.add_class("move_button");
        let style = include_bytes!("./style.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
        self.widgets.root.insert(&button, -1);

        if !white_player {
            self.model.move_number += 1;
            self.add_move_number();
        }

        self.widgets.root.show_all();
    }

    fn add_move_number(&mut self) {
        let button_label = format!("{}", self.model.move_number);
        let button = gtk::Button::with_label(&button_label);
        let style_context = button.style_context();
        style_context.add_class("number_button");
        let style = include_bytes!("./style.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
        self.widgets.root.insert(&button, -1);
    }
}

#[derive(Msg)]
pub enum Msg {
    AddMoveSan(String, bool),
}

pub struct Model {
    relm: Relm<History>,
    game_in_progress: bool,
    move_number: u16,
}

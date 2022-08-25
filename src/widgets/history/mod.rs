use gtk::{
    traits::{CssProviderExt, StyleContextExt, WidgetExt},
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

#[widget]
impl Widget for History {
    view! {
        #[name="root"]
        gtk::FlowBox {
            widget_name: "history_root"
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddMoveSan(san) => self.add_move_san(san),
        }
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            game_in_progress: false,
        }
    }

    fn init_view(&mut self) {
        let style_context = self.widgets.root.style_context();
        let style = include_bytes!("./style.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

impl History {
    fn add_move_san(&mut self, san: String) {}
}

#[derive(Msg)]
pub enum Msg {
    AddMoveSan(String),
}

pub struct Model {
    relm: Relm<History>,
    game_in_progress: bool,
}

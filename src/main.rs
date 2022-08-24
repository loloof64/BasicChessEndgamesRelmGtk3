use relm::Widget;
mod widgets;

use i18n_embed::{gettext::gettext_language_loader, DesktopLanguageRequester};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "locales/mo"]
struct Translations;

fn main() {
    let translations = Translations {};
    let language_loader = gettext_language_loader!();

    let requested_languages = DesktopLanguageRequester::requested_languages();

    i18n_embed::select(&language_loader, &translations, &requested_languages)
        .expect("Failed to embed translations.");

    widgets::mainwindow::MainWindow::run(()).unwrap();
}

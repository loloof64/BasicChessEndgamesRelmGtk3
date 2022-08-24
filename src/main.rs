use relm::Widget;
mod widgets;
mod translations;

fn main() {
    widgets::mainwindow::MainWindow::run(()).unwrap();
}

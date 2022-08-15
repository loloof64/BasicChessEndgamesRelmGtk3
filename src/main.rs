use relm::Widget;
mod widgets;

fn main() {
    widgets::mainwindow::MainWindow::run(()).unwrap();
}

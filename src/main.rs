use crate::ui::cli::start_ui;

mod backend {
    pub mod classes;
    pub mod enums;
}
mod ui;

fn main() {
    start_ui();
}
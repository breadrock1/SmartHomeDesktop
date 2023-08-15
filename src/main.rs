mod ui;

use crate::ui::HouseWidget;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    HouseWidget::run(Settings::default())
}

use bevy::prelude::*;
use bevy::prelude::Style;

pub const NORMAL_BUTTON_COLOUR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn create_normal_button_styles() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..Style::default()
    }
}

pub fn create_image_style() -> Style {
    Style {
        height: Val::Px(64.0),
        width: Val::Px(64.0),
        margin: UiRect {
            left: Val::Percent(8.0),
            right: Val::Percent(8.0),
            ..Style::DEFAULT.margin
        },
        ..Style::DEFAULT
    }
}

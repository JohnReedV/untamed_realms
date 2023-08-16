use bevy::{prelude::*, window::PrimaryWindow};

pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.6, 0.0, 0.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn main_menu_style(window_query: Query<&Window, With<PrimaryWindow>>) -> Style {
    let window = window_query.get_single().unwrap();

    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect {
            left: Val::Px((window.width() / 2.0) - 400.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
        },
        ..default()
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(75.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(5.0)),
        margin: UiRect {
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..default()
    }
}

pub fn title_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect {
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(100.0),
        },
        ..default()
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Righteous-Regular.ttf"),
        font_size: 128.0,
        color: Color::rgb(0.15, 0.15, 0.15),
    }
}

pub fn get_shadow_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Righteous-Regular.ttf"),
        font_size: 128.0,
        color: Color::BLACK,
    }
}
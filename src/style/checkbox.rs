use iced::border::{Border, Radius};
use iced::widget::checkbox::{Catalog, Status, Style, StyleFn};
use iced::{color, Color};

use super::helpers::border;
use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(entry)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn entry(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active { .. } | Status::Disabled { .. } => Style {
            background: p.middleground.into(),
            icon_color: p.accent,
            border: border(p.border),
            text_color: Some(p.text),
        },
        Status::Hovered { .. } => Style {
            background: p.foreground.into(),
            icon_color: p.accent,
            border: Border {
                color: p.accent,
                width: 2.0,
                radius: BORDER_RADIUS.into(),
            },
            text_color: Some(p.text),
        },
    }
}

pub fn inverted(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active { .. } | Status::Disabled { .. } | Status::Hovered { .. } => Style {
            background: p.foreground.into(),
            ..entry(theme, status)
        },
    }
}
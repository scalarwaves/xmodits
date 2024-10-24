use iced::border::{Border, Radius};
use iced::widget::text_input::{Catalog, Status, Style, StyleFn};
use iced::{color, Color};

use super::helpers::border;
use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active | Status::Disabled=> Style {
            background: p.foreground.into(),
            border: border(p.border),
            icon: p.foreground,
            placeholder: p.text,
            value: p.accent,
            selection: Color { a: 0.5, ..p.accent },
        },
        Status::Hovered | Status::Focused => Style {
            background: p.foreground.into(),
            border: border(p.accent),
            icon: p.foreground,
            placeholder: p.text,
            value: p.accent,
            selection: Color { a: 0.5, ..p.accent },
        },
    }
}

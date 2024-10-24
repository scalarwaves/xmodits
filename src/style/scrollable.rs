use iced::border::{Border, Radius};
use iced::widget::container;
use iced::widget::scrollable::{Catalog, Rail, Scrollbar, Scroller, Status, Style, StyleFn};
use iced::{color, Color};

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
    Style {
        container: Default::default(),
        vertical_rail: Rail {
            background: Default::default(),
            border: Default::default(),
            scroller: Scroller {
                color: Default::default(),
                border: Default::default(),
            },
        },
        horizontal_rail: Rail {
            background: Default::default(),
            border: Default::default(),
            scroller: Scroller {
                color: Default::default(),
                border: Default::default(),
            },
        },
        gap: Default::default(),
    }
    // match status {
    //     Status::Active => Style {
    //         container: todo!(),
    //         vertical_rail: todo!(),
    //         horizontal_rail: todo!(),
    //         gap: todo!(),
    //     },
    //     Status::Hovered { is_horizontal_scrollbar_hovered, is_vertical_scrollbar_hovered } =>  Style {
    //         container: todo!(),
    //         vertical_rail: todo!(),
    //         horizontal_rail: todo!(),
    //         gap: todo!(),
    //     },
    //     Status::Dragged { is_horizontal_scrollbar_dragged, is_vertical_scrollbar_dragged } =>  Style {
    //         container: todo!(),
    //         vertical_rail: todo!(),
    //         horizontal_rail: todo!(),
    //         gap: todo!(),
    //     },
    // }

    // match status {
    //     Status::Active => {
    //         let scrollbar = Scrollbar {
    //             background: Some(p.middleground.into()),
    //             border: Border {
    //                 color: Color::TRANSPARENT,
    //                 width: 0.0,
    //                 radius: 3.0.into(),
    //             },
    //             scroller: Scroller {
    //                 color: p.foreground,
    //                 border: Border {
    //                     color: p.border,
    //                     width: BORDER_WIDTH,
    //                     radius: 3.0.into(),
    //                 },
    //             },
    //             width: todo!(),
    //             margin: todo!(),
    //             scroller_width: todo!(),
    //             alignment: todo!(),
    //             spacing: todo!(),
    //         };

    //         Style {
    //             container: container::Style::default(),
    //             vertical_scrollbar: scrollbar,
    //             horizontal_scrollbar: scrollbar,
    //             gap: None,
    //         }
    //     }
    //     Status::Hovered {
    //         is_horizontal_scrollbar_hovered,
    //         is_vertical_scrollbar_hovered,
    //     } => {
    //         let scrollbar_hovered = is_horizontal_scrollbar_hovered | is_vertical_scrollbar_hovered;

    //         let scrollbar = Scrollbar {
    //             background: Some(p.middleground.into()),
    //             border: Border {
    //                 color: Color::TRANSPARENT,
    //                 width: 0.0,
    //                 radius: 3.0.into(),
    //             },
    //             scroller: Scroller {
    //                 color: if scrollbar_hovered {
    //                     Color { a: 0.5, ..p.accent }
    //                 } else {
    //                     p.foreground
    //                 },
    //                 border: Border {
    //                     color: if scrollbar_hovered {
    //                         Color {
    //                             a: 0.75,
    //                             ..p.accent
    //                         }
    //                     } else {
    //                         p.border
    //                     },
    //                     width: BORDER_WIDTH,
    //                     radius: 3.0.into(),
    //                 },
    //             },
    //             width: todo!(),
    //             margin: todo!(),
    //             scroller_width: todo!(),
    //             alignment: todo!(),
    //             spacing: todo!(),
    //         };

    //         Style {
    //             container: container::Style::default(),
    //             vertical_scrollbar: scrollbar,
    //             horizontal_scrollbar: scrollbar,
    //             gap: None,
    //         }
    //     }
    //     Status::Dragged {
    //         is_horizontal_scrollbar_dragged,
    //         is_vertical_scrollbar_dragged,
    //     } => primary(
    //         theme,
    //         Status::Hovered {
    //             is_horizontal_scrollbar_hovered: is_horizontal_scrollbar_dragged,
    //             is_vertical_scrollbar_hovered: is_vertical_scrollbar_dragged,
    //         },
    //     ),
    // }
}

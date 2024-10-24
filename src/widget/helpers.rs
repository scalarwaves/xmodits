//! Helper functions to construct widgets

use std::borrow::Borrow;

use iced::alignment::Horizontal;
use iced::widget::text::IntoFragment;
use iced::widget::{button, container, row, text};
use iced::{Alignment, Length};

use crate::style;
use crate::widget::{Button, Column, Container, Element, PickList, Text};

use super::Row;

/// TODO
pub fn centered_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content)
}
pub fn smol_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    button(content).height(Length::Shrink)
}

pub fn action<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    message: Option<Message>,
) -> Button<'a, Message> {
    button(content).on_press_maybe(message)
}

pub fn centered_text<'a>(input: impl IntoFragment<'a>) -> Text<'a> {
    text(input).align_x(Horizontal::Center)
}

pub fn warning<'a>(
    predicate: impl Fn() -> bool,
    warning: impl IntoFragment<'a>,
) -> Option<Text<'a>> {
    predicate().then_some(text(warning).style(style::text::warning))
}

pub fn centered_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
}

pub fn fill_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(content).width(Length::Fill).height(Length::Fill)
}

/// XMODITS control helper widget
pub fn control<'a, Message: 'a>(
    title: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(
        Column::new().spacing(8).push(title).push(
            container(content)
                .padding(8)
                .style(style::container::frame)
                .width(Length::Fill),
        ),
    )
}

pub fn control_filled<'a, Message: 'a>(
    title: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    fill_container(
        Column::new().spacing(8).push(title).push(
            container(content)
                .padding(8)
                .style(style::container::frame)
                .width(Length::Fill)
                .height(Length::Fill),
        ),
    )
}

pub fn labelled_picklist<'a, Message, T, L, V, F>(
    label: impl IntoFragment<'a>,
    options: L,
    selected: Option<V>,
    on_selected: F,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
    T: ToString + Eq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    F: Fn(T) -> Message + 'a,
{
    row![PickList::new(options, selected, on_selected), text(label)]
        .align_y(Alignment::Center)
        .spacing(8)
        .into()
}

pub fn centered_column<'a, Message>(column: Column<'a, Message>) -> Column<'a, Message> {
    column
        .spacing(5)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
}

pub fn centered_column_x<'a, Message>(column: Column<'a, Message>) -> Column<'a, Message> {
    column
        .spacing(5)
        .align_x(Alignment::Center)
        .width(Length::Fill)
}

pub fn spaced_row<'a, Message: 'a>(row: Row<'a, Message>) -> Row<'a, Message> {
    row.align_y(Alignment::Center).spacing(5)
}

pub fn text_icon<'a, Message: 'a>(text: &'a str, icon: Text<'a>) -> Row<'a, Message> {
    text_elem(text, icon).spacing(8)
}

pub fn text_icon_srnd<'a, Message: 'a>(text: &'a str, icon: Text<'a>) -> Row<'a, Message> {
    row![icon]
        .push(text)
        // .push(icon)
        .align_y(Alignment::Center)
        .spacing(5)
        .spacing(8)
}

pub fn text_elem<'a, Message: 'a>(
    text: &'a str,
    elem: impl Into<Element<'a, Message>>,
) -> Row<'a, Message> {
    row![text]
        .push(elem)
        .align_y(Alignment::Center)
        .spacing(5)
}

/// create text widget with advanced shaping for font fallback
pub fn text_adv<'a>(str: impl IntoFragment<'a>) -> Text<'a> {
    text(str).shaping(text::Shaping::Advanced)
}

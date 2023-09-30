//! The main display panel

pub mod entry;

use data::time::Time;
use entry::Entries;
use iced::{Alignment, Length};

use self::entry::Entry;

use crate::ripper::extraction::error::Reason;
use crate::ripper::subscription::CompleteState;
use crate::theme::Button;
use crate::widget::helpers::{centered_container, centered_text};
use crate::widget::{Collection, Element, Text};
use crate::{icon, theme};

use crate::app::Message;

use iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, text, Space,
};

pub fn view_samples<'a>() -> Element<'a, Message> {
    todo!()
}

pub fn view_entries(entries: &Entries) -> Element<Message> {
    let entries = &entries.entries;

    if entries.is_empty() {
        return centered_container(centered_text("Drag and Drop")).into();
    }

    scrollable(column(
        entries
            .iter()
            .enumerate()
            .map(|(index, entry)| view_entry(index, entry))
            .collect(),
    ))
    .into()
}

fn view_entry(index: usize, entry: &Entry) -> Element<Message> {
    let check = checkbox("", entry.selected, move |selected| Message::Select {
        index,
        selected,
    });

    let filename = text(&entry.filename());

    let view = row![check, filename]
        .push_maybe(match entry.is_dir() {
            true => Some(row![Space::with_width(Length::Fill), icon::folder()]),
            false => None,
        })
        .spacing(1)
        .align_items(Alignment::Center);

    row![
        button(view)
            .width(Length::Fill)
            .on_press(Message::Probe(index))
            .padding(4)
            .style(theme::Button::Entry),
        Space::with_width(15)
    ]
    .into()
}

pub fn view_ripping<'a>(
    message: &Option<String>,
    progress: f32,
    total_errors: u64,
) -> Element<'a, Message> {
    let cancel_ripping_button = button("Cancel").on_press(Message::Cancel).padding(5);

    let view = column![
        centered_text(message.as_deref().unwrap_or("Ripping...")),
        progress_bar(0.0..=100.0, progress).height(5).width(200),
        cancel_ripping_button,
        centered_text(format!("Errors: {}", total_errors)),
        // gif(&GIF.ripping)
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    centered_container(view).into()
}

/// XMODITS has finished extracting the samples
pub fn view_finished<'a>(
    complete_state: &'a CompleteState,
    time: &'a Time,
) -> Element<'a, Message> {
    let continue_button = button("Continue")
        // .on_press(Message::SetState(State::Idle))
        .padding(5);

    let save_errors_button = button("Save Errors")
        .on_press(Message::SaveErrors)
        .padding(5);

    match complete_state {
        CompleteState::NoErrors => centered_container(
            column![
                text("Done! \\(^_^)/"),
                text("Drag and Drop"),
                text(time),
                Space::with_height(15),
                continue_button
            ]
            .align_items(Alignment::Center),
        )
        .into(),

        CompleteState::Cancelled => centered_container(
            column![
                text("Cancelled"),
                text("Drag and Drop"),
                text(time),
                Space::with_height(15),
                continue_button
            ]
            .align_items(Alignment::Center),
        )
        .into(),

        // TODO
        CompleteState::Aborted => centered_container(
            column![
                text("An internal error occured."),
                Space::with_height(15),
                continue_button
            ]
            .align_items(Alignment::Center),
        )
        .into(),

        CompleteState::SomeErrors(errors) => {
            let message = column![
                centered_text("Done... But xmodits could not rip everything... (._.)"),
                text("took...")
            ];

            let buttons = row![continue_button, save_errors_button]
                .padding(4)
                .spacing(6)
                .align_items(Alignment::Center);

            let errors = scrollable(column(
                errors
                    .iter()
                    .map(|error| {
                        let reason = match &error.reason {
                            Reason::Single(single) => text(single),
                            Reason::Multiple(multiple) => text("multiple..."), //todo
                        };

                        let error = text(error.filename());
                        container(column![error, reason]).into()
                    })
                    .collect(),
            ));

            let view = column![message, buttons, errors];

            centered_container(view).into()
        }

        CompleteState::TooMuchErrors { log, total } => {
            let view = column![
                text("Done..."),
                text("But there's too many errors to display! (-_-')"),
                text("Check the logs at:"),
                button(centered_text(log.display()))
                    .on_press(Message::Open(log.display().to_string())),
                centered_text(format!("{} errors written", total)),
                centered_text(time),
                // space,
                row![continue_button]
                    .padding(4)
                    .align_items(Alignment::Center)
            ]
            .align_items(Alignment::Center)
            .padding(4)
            .spacing(6);

            centered_container(view).into()
        }

        CompleteState::TooMuchErrorsNoLog {
            reason,
            errors,
            discarded,
            manually_saved,
        } => {
            let error_message = match errors.len() {
                0 => match manually_saved {
                    false => text("Manually Saving errors..."),
                    true => text("Errors saved manually :D"),
                },
                n => text(format!("{} stored errors", n)),
            };

            let discarded_errors = match discarded {
                0 => text("No errors were discarded."),
                n => text(format!(
                    "I had to discard {} error(s) to save memory. >_<",
                    n
                )), // .style(style::text::Text::Error),
            };

            let buttons = match manually_saved {
                true => row![continue_button],
                false => row![continue_button, save_errors_button],
            }
            .padding(4)
            .spacing(6)
            .align_items(Alignment::Center);

            let view = column![
                text("Done..."),
                text("But there's too many errors to display! (-_-')"),
                text("...and I can't store them to a file either:"),
                centered_text(format!("\"{}\"", reason)),
                // .style(style::text::Text::Error),
                buttons,
                error_message,
                discarded_errors,
            ]
            .align_items(Alignment::Center)
            .padding(4)
            .spacing(6);

            centered_container(view).into()
        }
    }
}

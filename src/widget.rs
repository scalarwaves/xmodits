//! Custom widgets
//!
//! Most it graciously yoinked from: https://github.com/squidowl/halloy/
//!
//! I cannot express my gratitude enough. Those guys are awesome.

pub mod animation;
pub mod helpers;

#[cfg(feature = "audio")]
pub mod waveform_view;

pub type Renderer = iced::Renderer;
pub type Theme = crate::style::Theme;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
pub type Content<'a, Message> = iced::widget::pane_grid::Content<'a, Message, Theme, Renderer>;
pub type TitleBar<'a, Message> = iced::widget::pane_grid::TitleBar<'a, Message, Theme, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Theme, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Theme, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Theme, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Theme, Renderer>;
pub type PickList<'a, Message, T, L, V> = iced::widget::PickList<'a, T, L, V, Message, Theme, Renderer>;

#[cfg(feature = "audio")]
pub type WaveformViewer<'a, Message> = waveform_view::WaveformViewer<'a, Message, Theme>;

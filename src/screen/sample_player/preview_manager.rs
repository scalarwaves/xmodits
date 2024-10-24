use super::instance::{self, Instance, MediaSettings};

use iced::window::{self, Id};
use iced::{Size, Task};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::app::application_icon;
use crate::screen::entry::Entries;
use crate::widget::Element;

use audio_engine::SamplePlayer;

const WINDOW_SIZE: Size = Size::new(640.0, 500.0);

#[derive(Debug, Clone)]
pub enum Message {
    Window(Id, instance::Message),
    WindowOpened(Id, PathBuf),
}

#[derive(Default)]
pub struct SamplePreview {
    audio_engine: SamplePlayer,
    windows: HashMap<Id, Instance>,
    default_settings: MediaSettings,
}

impl SamplePreview {
    pub fn update(&mut self, msg: Message, entries: &mut Entries) -> Task<Message> {
        match msg {
            Message::Window(id, msg) => self.update_window(id, msg, entries),
            Message::WindowOpened(id, path) => {
                let (instance, load_samples) =
                    Instance::new(self.audio_engine.create_handle(), path);

                self.windows
                    .insert(id, instance.settings(self.default_settings));

                load_samples.map(move |msg| Message::Window(id, msg))
            }
        }
    }

    pub fn update_window(
        &mut self,
        id: Id,
        msg: instance::Message,
        entries: &mut Entries,
    ) -> Task<Message> {
        // If the window has closed, discard the message
        match self.windows.get_mut(&id) {
            None => Task::none(),
            Some(window) => window
                .update(msg, entries)
                .map(move |msg| Message::Window(id, msg)),
        }
    }

    pub fn view(&self, id: Id, entries: &Entries) -> Element<Message> {
        self.get_window(id)
            .view(entries)
            .map(move |msg| Message::Window(id, msg))
    }

    pub fn remove_instance(&mut self, id: Id) {
        self.windows.remove_entry(&id);
    }

    // spawn new instance
    pub fn create_instance(&mut self, path: PathBuf) -> Task<Message> {
        match self.find(&path) {
            Some(old_id) => window::gain_focus(old_id),
            None => self.new_instance(path),
        }
    }

    fn new_instance(&mut self, path: PathBuf) -> Task<Message> {
        window::open(window::Settings {
            size: WINDOW_SIZE,
            min_size: Some(WINDOW_SIZE),
            icon: Some(application_icon()),
            exit_on_close_request: true,
            ..Default::default()
        })
        .map(move |id| Message::WindowOpened(id, path.clone()))
    }

    pub fn get_title(&self, id: Id) -> String {
        self.get_window(id).title()
    }

    pub fn set_hovered(&mut self, id: Id, hovered: bool) {
        self.get_window_mut(id).hovered = hovered;
    }

    pub fn load_samples(&mut self, id: Id, path: PathBuf) -> Task<Message> {
        match self.find(&path) {
            Some(old_id) if old_id != id => window::gain_focus(old_id),
            _ => self
                .get_window_mut(id)
                .load_samples(path)
                .map(move |result| Message::Window(id, result)),
        }
    }

    // find a window that already has a tracker loaded
    pub fn find(&self, path: &Path) -> Option<Id> {
        self.windows
            .iter()
            .find_map(|(id, window)| window.matches_path(path).then_some(id))
            .copied()
    }

    pub fn get_window(&self, id: Id) -> &Instance {
        self.windows.get(&id).expect("View sample preview window")
    }

    pub fn get_window_mut(&mut self, id: Id) -> &mut Instance {
        self.windows
            .get_mut(&id)
            .expect("View sample preview window")
    }

    pub fn close_all(&mut self) -> Task<Message> {
        let command = Task::batch(self.windows.keys().map(|id| window::close(*id)));
        self.windows.clear();
        command
    }

    pub fn instances(&self) -> usize {
        self.windows.len()
    }

    pub fn find_first_instance(&self) -> Option<Id> {
        self.windows.keys().next().copied()
    }
}

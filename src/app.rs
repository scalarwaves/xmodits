mod simple;

use std::path::PathBuf;

use crate::event;
use crate::font;
use crate::icon;
use crate::logger;
use crate::ripper::subscription::CompleteState;
use crate::ripper::{self, Message as RipperMessage};
use crate::screen::config::advanced::{self, AdvancedConfig};
use crate::screen::config::sample_naming::{self, NamingConfig};
use crate::screen::config::sample_ripping::{self, RippingConfig, DESTINATION_BAR_ID};
use crate::screen::history::History;
use crate::screen::tracker_info::{self, TrackerInfo};
use crate::screen::{about, build_info, main_panel::entry::Entries};
use crate::theme;
use crate::utils::{files_dialog, folders_dialog};
use crate::widget::{Collection, Column, Container, Element};

use data::Config;

use iced::widget::text_input;
use iced::widget::{button, column, row, Space};
use iced::{window, Event as IcedEvent, Length};
use iced::{Application, Command, Settings, Subscription};

/// XMODITS graphical application
#[derive(Default)]
pub struct XMODITS {
    entries: Entries,
    history: History,
    state: State,
    view: View,
    ripper: ripper::Handle,
    #[cfg(feature = "audio")]
    audio: audio_engine::Handle,
    tracker_info: Option<TrackerInfo>,
    // sample_pack: (),
    theme: theme::Theme,
    naming_cfg: NamingConfig,
    ripping_cfg: RippingConfig,
    advanced_cfg: AdvancedConfig,
}

impl XMODITS {
    /// Launch the application
    pub fn launch() -> iced::Result {
        // Setup logging stuff
        logger::set_panic_hook();
        logger::init_logging();

        // load configuration
        let config = Config::load();

        //
        tracing::info!("Launcing GUI");
        Self::run(settings(config))
    }

    /// WINDOWS ONLY
    ///
    /// XMODITS' simple mode to allow dragging and dropping modules onto the binary
    #[cfg(windows)]
    pub fn launch_simple() {
        simple::rip(std::env::args())
    }

    pub fn settings() {}

    pub fn build_start_signal(&mut self) -> ripper::Signal {
        let entries = self.entries.take();
        let ripping = self.ripping_cfg.0.to_owned();
        let naming = self.naming_cfg.0.to_owned();

        ripper::Signal {
            entries,
            ripping,
            naming,
        }
    }
}

/// TODO: allow the user to customize their application icon
fn icon() -> iced::window::Icon {
    let icon = include_bytes!("../assets/img/logo/icon2.png");
    iced::window::icon::from_file_data(icon, None).unwrap()
}

pub fn settings(config: Config) -> iced::Settings<Config> {
    iced::Settings {
        default_font: font::JETBRAINS_MONO,
        default_text_size: 13.0.into(),
        exit_on_close_request: true,
        flags: config,
        window: window::Settings {
            icon: Some(icon()),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// The current state of the application.
#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    /// The user is previewing some samples
    SamplePreview(/* TODO */),
    /// The application is currently ripping samples
    Ripping {
        message: Option<String>,
        progress: f32,
        errors: u64,
    },
    /// The application has finished ripping samples
    Finished {
        state: CompleteState,
        time: data::Time,
    },
}

impl State {
    fn update_progress(&mut self, new_progress: f32, new_errors: u64) {
        if let Self::Ripping {
            progress, errors, ..
        } = self
        {
            *progress = new_progress;
            *errors = new_errors;
        }
    }

    fn update_message(&mut self, new_message: Option<String>) {
        if let Self::Ripping { message, .. } = self {
            *message = new_message
        }
    }

    fn set_message(&mut self, message: impl Into<String>) {
        self.update_message(Some(message.into()))
    }

    fn is_ripping(&self) -> bool {
        matches!(self, Self::Ripping { .. })
    }
}

/// TODO: rename to avoid confusion
///
/// This is basically the configuration panel view.
#[derive(Default, Debug, Clone)]
pub enum View {
    #[default]
    Configure,
    Settings,
    About,
    Help,
}

#[derive(Debug, Clone)]
pub enum Message {
    About(about::Message),
    AboutPressed,
    Add(PathBuf),
    AddMultiple(Option<Vec<PathBuf>>),
    AdvancedCfg(advanced::Message),
    #[cfg(feature = "audio")]
    Audio(),
    Cancel,
    Clear,
    ConfigPressed,
    DeleteSelected,
    Event(event::Event),
    FileDialog,
    FolderDialog,
    FontsLoaded(Result<(), iced::font::Error>),
    Ignore,
    InvertSelection,
    NamingCfg(sample_naming::Message),
    Open(String),
    Probe(usize),
    ProbeResult(TrackerInfo),
    RippingCfg(sample_ripping::Message),
    SaveConfig,
    SaveConfigResult(),
    SaveErrors,
    SaveErrorsResult(),
    StartRipping,
    Subscription(RipperMessage),
}

impl Application for XMODITS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = Config;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), font::load().map(Message::FontsLoaded))
    }

    fn title(&self) -> String {
        String::from("XMDOITS - 10%") // todo: add status
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::About(msg) => about::update(msg),
            Message::AboutPressed => self.view = View::About,
            Message::Add(path) => {
                if self.state.is_ripping() {
                    return Command::none();
                }

                self.entries.add(path)
            }
            Message::AddMultiple(paths) => {
                if self.state.is_ripping() {
                    return Command::none();
                }

                if let Some(paths) = paths {
                    self.entries.add_multiple(paths)
                }
                // todo: change state
            }
            Message::AdvancedCfg(msg) => self.advanced_cfg.update(msg),
            Message::Cancel => {
                self.state.set_message("Cancelling...");
                self.ripper.cancel();
            }
            Message::Clear => {
                // todo change state. clear current loaded module
                self.entries.clear();
            }
            Message::ConfigPressed => self.view = View::Configure,
            Message::DeleteSelected => self.entries.delete_selected(),
            Message::Event(event) => match event {
                event::Event::Clear => self.entries.clear(),
                event::Event::CloseRequested => {}
                event::Event::Delete => self.entries.delete_selected(),
                event::Event::FileDropped(file) => self.entries.add(file),
                event::Event::Save => {}
                event::Event::Start => {}
            },
            Message::FileDialog => return Command::perform(files_dialog(), Message::AddMultiple),
            Message::FolderDialog => {
                return Command::perform(folders_dialog(), Message::AddMultiple)
            }
            Message::FontsLoaded(result) => {
                if result.is_err() {
                    tracing::error!("could not load font")
                }
            }
            Message::Ignore => (),
            Message::RippingCfg(msg) => {
                return self.ripping_cfg.update(msg).map(Message::RippingCfg)
            }
            Message::InvertSelection => self.entries.invert(),
            Message::NamingCfg(msg) => self.naming_cfg.update(msg),
            Message::Open(link) => {
                if let Err(err) = open::that_detached(link) {
                    tracing::warn!("Could not open external link: {:?}", err)
                };
            }
            Message::Probe(idx) => {
                let path = self.entries.get(idx).unwrap().to_owned();
                return Command::perform(tracker_info::probe(path), Message::ProbeResult);
            }
            Message::ProbeResult(probe) => self.tracker_info = Some(probe),
            Message::SaveConfig => {}
            Message::SaveConfigResult() => {}
            Message::SaveErrors => todo!(),
            Message::SaveErrorsResult() => todo!(),
            Message::StartRipping => {
                if self.state.is_ripping() | self.entries.is_empty() | !self.ripper.is_active() {
                    return Command::none();
                }

                if !self.ripping_cfg.destination_is_valid() {
                    tracing::error!(
                        "The provided destination is not valid. The *parent* folder must exist."
                    );
                    return text_input::focus(DESTINATION_BAR_ID.clone());
                }

                let start_signal = self.build_start_signal();
                let _ = self.ripper.send(start_signal);

                self.state = State::Ripping {
                    message: None,
                    progress: 0.0,
                    errors: 0,
                }
            }
            Message::Subscription(msg) => match msg {
                RipperMessage::Ready(sender) => self.ripper.set_sender(sender),
                RipperMessage::Info(info) => self.state.update_message(info),
                RipperMessage::Progress { progress, errors } => {
                    self.state.update_progress(progress, errors)
                }
                RipperMessage::Done { state, time } => {
                    self.ripper.reset_stop_flag(); // todo: should this be here?
                    self.state = State::Finished { state, time };
                    tracing::debug!("{:#?}", self.state);
                }
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_left_menu = row![
            button("Configure").on_press(Message::ConfigPressed),
            button("Settings").on_press(Message::ConfigPressed),
            button("About").on_press(Message::AboutPressed),
        ];

        let bottom_left_buttons = row![
            button("Save Configuration").on_press(Message::SaveConfig),
            button(row!["START", icon::download()]).on_press(Message::StartRipping),
        ];

        let left_half = column![
            top_left_menu,
            self.naming_cfg.view().map(Message::NamingCfg),
            self.ripping_cfg.view().map(Message::RippingCfg),
            bottom_left_buttons,
        ];

        let destination =
            sample_ripping::view_destination_bar(&self.ripping_cfg).map(Message::RippingCfg);

        let right_bottom_buttons = row![
            button("Add File").on_press(Message::FileDialog),
            button("Add Folder").on_press(Message::FolderDialog),
            Space::with_width(Length::Fill),
            button("Delete Selected").on_press(Message::DeleteSelected),
            button("Clear").on_press(Message::Clear),
        ];

        let right_half = column![
            destination,
            // self.entries
            right_bottom_buttons
        ];

        let main = row![left_half, right_half];

        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch([
            event::events().map(Message::Event),
            ripper::xmodits_subscription().map(Message::Subscription),
        ])
    }
}

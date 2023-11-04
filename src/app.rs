#[cfg(windows)]
mod simple;

use std::path::PathBuf;

use crate::event;
use crate::font;
use crate::icon;
use crate::logger;
use crate::ripper;
use crate::ripper::subscription::{CompleteState, ErrorHandler};
use crate::screen::config::custom_filters;
use crate::screen::config::name_preview;
use crate::screen::config::sample_naming;
use crate::screen::config::sample_ripping::{self, DESTINATION_BAR_ID};
use crate::screen::history::History;
use crate::screen::main_panel;
use crate::screen::settings;
use crate::screen::tracker_info::{self, TrackerInfo};
use crate::screen::{about, main_panel::entry::Entries};
use crate::theme;
use crate::utils::{create_file_dialog, files_dialog, folders_dialog};
use crate::widget::helpers::{action, text_icon, warning};
use crate::widget::{Collection, Container, Element};

use data::Config;

use iced::widget::{button, checkbox, column, row, text, text_input, Space};
use iced::Alignment;
use iced::{window, Application, Command, Length, Subscription};

const TITLE: &str = "XMODITS";

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
    naming_cfg: data::config::SampleNameConfig,
    ripping_cfg: data::config::SampleRippingConfig,
    general_cfg: data::config::GeneralConfig,
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
    pub fn launch_simple(paths: impl IntoIterator<Item = String>) -> iced::Result {
        simple::rip(paths);
        Ok(())
    }

    pub fn settings() {}

    pub fn load_cfg(&mut self, config: Config) {
        // todo
        self.ripping_cfg = config.ripping;
        self.naming_cfg = config.naming;
        self.general_cfg = config.general;
    }

    pub fn build_start_signal(&mut self) -> ripper::Signal {
        let entries = self.entries.take();
        let ripping = self.ripping_cfg.to_owned();
        let naming = self.naming_cfg.to_owned();

        ripper::Signal {
            entries,
            ripping,
            naming,
        }
    }

    pub fn clear_entries(&mut self) {
        self.tracker_info = None;
        self.entries.clear();
    }

    pub fn delete_selected_entries(&mut self) {
        let current_tracker_path = self.tracker_info.as_ref().map(|f| f.path());
        let clear_tracker_info = self.entries.delete_selected(current_tracker_path);

        if clear_tracker_info {
            self.tracker_info = None;
        }
    }

    pub fn app_title(&self) -> String {
        let modifiers: Option<String> = match &self.state {
            State::Idle | State::SamplePreview(..) | State::Finished { .. } => None,
            State::Ripping {
                message, progress, ..
            } => Some(format!(
                "{} - {}%",
                message.as_deref().unwrap_or("Ripping..."),
                progress.floor()
            )),
        };

        match modifiers {
            Some(modi) => format!("{TITLE} - {modi}"),
            None => TITLE.to_string(),
        }
    }

    pub fn save_cfg(&self) -> Command<Message> {
        let config = data::Config {
            general: self.general_cfg.clone(),
            ripping: self.ripping_cfg.clone(),
            naming: self.naming_cfg,
        };

        Command::perform(async move { config.save().await }, |_| Message::Ignore)
    }

    pub fn start_ripping(&mut self) -> Command<Message> {
        if self.state.is_ripping() | self.entries.is_empty() | !self.ripper.is_active() {
            return Command::none();
        }

        if !sample_ripping::destination_is_valid(&self.ripping_cfg) {
            tracing::error!("The provided destination is not valid. The *parent* folder must exist.");
            return text_input::focus(DESTINATION_BAR_ID.clone());
        }

        let start_signal = self.build_start_signal();
        let _ = self.ripper.send(start_signal);

        self.state = State::Ripping {
            message: None,
            progress: 0.0,
            errors: 0,
        };

        Command::none()
    }
}

/// TODO: allow the user to customize their application icon
fn icon() -> iced::window::Icon {
    let icon = include_bytes!("../assets/img/logos/icon.png");
    iced::window::icon::from_file_data(icon, None).unwrap()
}

pub fn settings(config: Config) -> iced::Settings<Config> {
    iced::Settings {
        fonts: vec![
            include_bytes!("../assets/font/icons.ttf").as_slice().into(),
            include_bytes!("../assets/font/JetBrainsMono-Regular.ttf")
                .as_slice()
                .into(),
        ],
        default_font: font::JETBRAINS_MONO,
        default_text_size: 13.0.into(),
        exit_on_close_request: true,
        flags: config,
        window: window::Settings {
            icon: Some(icon()),
            size: (780, 720),
            min_size: Some((780, 720)),
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
    Finished { state: CompleteState, time: data::Time },
}

impl State {
    fn update_progress(&mut self, new_progress: f32, new_errors: u64) {
        if let Self::Ripping { progress, errors, .. } = self {
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
    Filters,
    Settings,
    About,
    Help,
}

#[derive(Debug, Clone)]
pub enum Message {
    AboutPressed,
    Add(Option<Vec<PathBuf>>),
    #[cfg(feature = "audio")]
    Audio(),
    Cancel,
    Clear,
    ConfigPressed,
    DeleteSelected,
    Event(event::Event),
    FileDialog,
    FilterPressed,
    FolderDialog,
    GeneralCfg(settings::Message),
    HistoryPressed,
    Ignore,
    InvertSelection,
    NamingCfg(sample_naming::Message),
    Open(String),
    PreviewSamples(PathBuf),
    Probe(usize),
    ProbeResult(TrackerInfo),
    RippingCfg(sample_ripping::Message),
    SaveConfig,
    SaveConfigResult(),
    SaveErrors,
    SaveErrorsResult(Result<(), String>),
    Select {
        index: usize,
        selected: bool,
    },
    SelectAll(bool),
    SetState(State),
    SetTheme,
    SettingsPressed,
    StartRipping,
    Subscription(ripper::Message),
}

impl Application for XMODITS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let mut app = Self::default();
        app.load_cfg(flags);

        (app, Command::none())
    }

    fn title(&self) -> String {
        self.app_title()
    }

    fn theme(&self) -> Self::Theme {
        // todo...
        theme::Theme(self.general_cfg.theme.palette()).clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AboutPressed => self.view = View::About,
            Message::ConfigPressed => self.view = View::Configure,
            Message::FilterPressed => self.view = View::Filters,
            Message::SettingsPressed => self.view = View::Settings,
            Message::HistoryPressed => {}
            Message::Add(paths) => {
                if self.state.is_ripping() {
                    return Command::none();
                }

                if let Some(paths) = paths {
                    self.entries.add_multiple(paths)
                }
                // todo: change state to idle?
            }
            Message::Clear => self.clear_entries(),
            Message::DeleteSelected => self.delete_selected_entries(),
            Message::InvertSelection => self.entries.invert(),
            Message::Select { index, selected } => self.entries.select(index, selected),
            Message::SelectAll(selected) => self.entries.select_all(selected),
            Message::SetState(state) => self.state = state,
            Message::FileDialog => {
                return Command::perform(files_dialog(), Message::Add);
            }
            Message::FolderDialog => {
                return Command::perform(folders_dialog(), Message::Add);
            }
            Message::GeneralCfg(cfg) => {
                return settings::update(&mut self.general_cfg, cfg).map(Message::GeneralCfg)
            }
            Message::RippingCfg(msg) => {
                return sample_ripping::update(&mut self.ripping_cfg, msg).map(Message::RippingCfg)
            }
            Message::NamingCfg(msg) => sample_naming::update(&mut self.naming_cfg, msg),
            Message::SetTheme => todo!(),
            Message::Open(link) => {
                if let Err(err) = open::that_detached(link) {
                    tracing::warn!("Could not open external link: {:?}", err)
                };
            }
            Message::PreviewSamples(_) => (),
            Message::Probe(idx) => {
                let path = self.entries.get(idx).unwrap();

                if self
                    .tracker_info
                    .as_ref()
                    .is_some_and(|info| info.matches_path(path))
                    | path.is_dir()
                {
                    return Command::none();
                }

                return Command::perform(tracker_info::probe(path.to_owned()), Message::ProbeResult);
            }
            Message::ProbeResult(probe) => self.tracker_info = Some(probe),
            Message::SaveConfig => {
                return self.save_cfg();
            }
            Message::SaveConfigResult() => {}
            Message::SaveErrors => {
                let State::Finished { state, .. } = &mut self.state else {
                    return Command::none();
                };

                let Some(errors) = state.errors_ref_mut().cloned() else {
                    return Command::none();
                };

                return Command::perform(
                    async move {
                        let Some(path) = create_file_dialog().await else {
                            return Err(String::new()); // todo
                        };

                        ErrorHandler::dump(errors, path).await
                    },
                    Message::SaveErrorsResult,
                );
            }
            Message::SaveErrorsResult(result) => {
                if result.is_ok() {
                    self.state = State::Idle;
                }
            }
            Message::StartRipping => {
                return self.start_ripping();
            }
            Message::Cancel => {
                self.state.set_message("Cancelling...");
                self.ripper.cancel();
            }
            Message::Event(event) => match event {
                event::Event::Clear => self.clear_entries(),
                event::Event::CloseRequested => {}
                event::Event::Delete => self.delete_selected_entries(),
                event::Event::FileDropped(file) => self.entries.add(file),
                event::Event::Save => return self.save_cfg(),
                event::Event::Start => return self.start_ripping(),
            },
            Message::Subscription(msg) => match msg {
                ripper::Message::Ready(sender) => self.ripper.set_sender(sender),
                ripper::Message::Info(info) => self.state.update_message(info),
                ripper::Message::Progress { progress, errors } => {
                    self.state.update_progress(progress, errors)
                }
                ripper::Message::Done { state, time } => {
                    self.ripper.reset_stop_flag(); // todo: should this be here?
                    self.state = State::Finished { state, time };
                }
            },
            Message::Ignore => (),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_left_menu = row![
            button("Configure").on_press(Message::ConfigPressed),
            button("Filters").on_press(Message::FilterPressed),
            // button("History").on_press(Message::HistoryPressed),
            button("Settings").on_press(Message::SettingsPressed),
            button("About").on_press(Message::AboutPressed),
        ]
        .spacing(5)
        .width(Length::Fill)
        .align_items(Alignment::Center);

        let not_ripping = !self.state.is_ripping();

        let bottom_left_buttons = row![
            button(text_icon("Save Settings", icon::save()))
                .on_press(Message::SaveConfig)
                .width(Length::FillPortion(2)),
            button(text_icon("START", icon::download()))
                .on_press_maybe(not_ripping.then_some(Message::StartRipping))
                .style(theme::Button::Start)
                .width(Length::FillPortion(2))
        ]
        .spacing(8);

        let left_view = match self.view {
            View::Configure => {
                let naming_cfg = {
                    let name_preview = name_preview::preview_name(
                        &self.general_cfg.sample_name_params,
                        &self.naming_cfg,
                        &self.ripping_cfg,
                    );

                    sample_naming::view(&self.naming_cfg, name_preview).map(Message::NamingCfg)
                };

                let ripping_cfg = sample_ripping::view(&self.ripping_cfg).map(Message::RippingCfg);

                column![
                    tracker_info::view(self.tracker_info.as_ref()),
                    naming_cfg,
                    ripping_cfg,
                    bottom_left_buttons,
                ]
                .spacing(8)
                .into()
            }
            View::Filters => column![
                custom_filters::view().map(|_| Message::Ignore),
                bottom_left_buttons,
            ]
            .spacing(8)
            .into(),
            View::Settings => settings::view(&self.general_cfg).map(Message::GeneralCfg),
            View::About => about::view(),
            View::Help => todo!(),
        };

        let left_half = column![top_left_menu, Space::with_height(2), left_view]
            .width(Length::FillPortion(4))
            .spacing(10);

        let destination = sample_ripping::view_destination_bar(&self.ripping_cfg).map(Message::RippingCfg);

        let top_right_buttons = row![
            text(format!("Entries: {}", self.entries.len())),
            text(format!("Selected: {}", self.entries.total_selected())),
            Space::with_width(Length::Fill),
            button("Invert").on_press(Message::InvertSelection),
            checkbox("Select All", self.entries.all_selected, Message::SelectAll)
        ]
        .spacing(15)
        .align_items(Alignment::Center);

        let bottom_right_buttons = row![
            action("Add File", not_ripping.then_some(Message::FileDialog)),
            action("Add Folder", not_ripping.then_some(Message::FolderDialog)),
            Space::with_width(Length::Fill),
            action("Delete Selected", not_ripping.then_some(Message::DeleteSelected)),
            action("Clear", not_ripping.then_some(Message::Clear)),
        ]
        .spacing(5);

        let main_view = match &self.state {
            State::Idle => main_panel::view_entries(&self.entries),
            State::SamplePreview() => todo!(), // list samples
            State::Ripping {
                message,
                progress,
                errors,
            } => main_panel::view_ripping(message, *progress, *errors),
            State::Finished { state, time } => main_panel::view_finished(state, time),
        };

        let allow_warnings = !self.general_cfg.suppress_warnings;

        let bad_cfg_warning = warning(
            || allow_warnings && (!self.ripping_cfg.self_contained && !self.naming_cfg.prefix),
            "\"Self Contained\" is disabled. You should enable \"Prefix Samples\" to reduce collisions. Unless you know what you are doing."
        );

        let too_many_files_warning = warning(
            || allow_warnings && self.entries.len() > 200,
            "That's a lot of files! You REALLY should be using folders.",
        );

        let right_half = column![destination, top_right_buttons, main_view]
            .push_maybe(bad_cfg_warning)
            .push_maybe(too_many_files_warning)
            .push(bottom_right_buttons)
            .width(Length::FillPortion(5))
            .spacing(10);

        let main = row![left_half, right_half].spacing(10);

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

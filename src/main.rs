use std::collections::HashSet;
use std::collections::HashMap;
use std::path::Path;
use rust_iso3166::{ from_alpha2, ALL };
use rust_embed::Embed;
use image::GenericImageView;
use iced::{ Length, Pixels, Size, Task, Theme, window, Color, Background, Shadow };
use simple_logger::{ self, SimpleLogger };

use iced::widget::{ Space, TextInput, button, column, pick_list, row, text, text_input };
use cli_clipboard::{ ClipboardContext, ClipboardProvider };
use serde::Deserialize;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/data"]
#[include = "*.json"]
struct Asset;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/media"]
#[include = "*.png"]
struct Media;

#[derive(Deserialize)]
struct Consist {
    max_speed: u16,
    mass: u16,
    brake_force: u16,
    power: u16,
}

#[derive(Default)]
struct FormData {
    reference: String,
    description: String,
    start_speed: u16,
    consist: Option<String>,
    country_code: Option<String>,
    consist_list: HashMap<String, Consist>,
    consist_options: Vec<String>,
    current_header: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum StringInput {
    Reference,
    Description,
    Country,
    Consist,
}

#[derive(Debug, Clone, Copy)]
enum IntInput {
    StartSpeed,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(StringInput, String),
    NumericInputChanged(IntInput, u16),
    CopyTrigger,
}

fn plain_grey(_theme: &Theme, status: button::Status) -> button::Style {
    let active = button::Style {
        background: Some(Background::Color(Color::from_rgb8(240, 240, 240))),
        text_color: Color::BLACK,
        shadow: Shadow::default(),
        ..button::Style::default()
    };

    match status {
        button::Status::Hovered =>
            button::Style {
                background: Some(Background::Color(Color::from_rgb8(225, 225, 225))),
                ..active
            },
        button::Status::Pressed =>
            button::Style {
                background: Some(Background::Color(Color::from_rgb8(210, 210, 210))),
                ..active
            },
        _ => active,
    }
}

fn load_data(country_code: String) -> HashMap<String, Consist> {
    log::debug!("Loading country data for {country_code}");
    let mut consist_data: HashMap<String, Consist> = HashMap::new();

    let prefix = format!("{}/", country_code);

    for file_path in Asset::iter() {
        if file_path.starts_with(&prefix) && file_path.ends_with(".json") {
            if let Some(embedded_file) = Asset::get(file_path.as_ref()) {
                let file_consists: Result<HashMap<String, Consist>, _> = serde_json::from_slice(
                    &embedded_file.data
                );

                match file_consists {
                    Ok(consist) => {
                        consist_data.extend(consist);
                    }
                    Err(e) => {
                        log::error!("Failed to parse JSON for {}: {}", file_path, e);
                    }
                }
            }
        }
    }

    if consist_data.len() < 1 {
        log::warn!("No data found for '{}'", country_code);
    }

    consist_data
}

fn get_country_codes() -> Vec<String> {
    let mut country_codes: HashSet<String> = HashSet::new();

    for file in Asset::iter() {
        let path = Path::new(file.as_ref());

        if
            let Some(country_name) = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|f| f.to_str())
                .and_then(|n| from_alpha2(n.to_uppercase().as_str()))
        {
            country_codes.insert(country_name.name.to_string());
        }
    }

    country_codes.into_iter().collect()
}

fn load_icon() -> Option<window::Icon> {
    let image = match Media::get("RailOSConsist.png") {
        Some(f) => {
            match image::load_from_memory(&f.data) {
                Ok(o) => o,
                Err(_) => {
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    };
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8().into_raw();

    match window::icon::from_rgba(rgba, width, height) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

impl FormData {
    fn new() -> (Self, Task<Message>) {
        (
            FormData {
                reference: "1A00".to_string(),
                description: String::new(),
                start_speed: 0,
                consist: None,
                country_code: None,
                consist_list: HashMap::new(),
                consist_options: Vec::new(),
                current_header: None,
            },
            Task::none(),
        )
    }
    fn generate_header(&self) -> Option<String> {
        if self.reference.is_empty() || self.description.is_empty() {
            return None;
        }
        if let Some(consist) = self.consist.clone() {
            return Some(
                format!(
                    "{};{};{};{};{};{};{}",
                    self.reference,
                    self.description,
                    self.start_speed,
                    self.consist_list[&consist].max_speed,
                    self.consist_list[&consist].mass,
                    self.consist_list[&consist].brake_force,
                    self.consist_list[&consist].power
                )
            );
        }
        None
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextInputChanged(field, value) => {
                log::debug!("Setting {:?} to '{value}'", field);
                match field {
                    StringInput::Consist => {
                        self.consist = Some(value);
                        self.current_header = self.generate_header();
                    }
                    StringInput::Country => {
                        if let Some(country) = ALL.iter().find(|c| c.name == value) {
                            self.country_code = Some(country.name.to_string());
                            self.consist = None;
                            self.consist_list = load_data(country.alpha2.to_lowercase());
                            self.consist_options = self.consist_list
                                .keys()
                                .cloned()
                                .into_iter()
                                .collect();
                        }
                    }
                    StringInput::Description => {
                        self.description = value;
                        self.current_header = self.generate_header();
                    }
                    StringInput::Reference => {
                        self.reference = value;
                        self.current_header = self.generate_header();
                    }
                }
                Task::none()
            }
            Message::NumericInputChanged(field, value) => {
                log::debug!("Setting {:?} to '{value}'", field);
                match field {
                    IntInput::StartSpeed => {
                        self.start_speed = value;
                    }
                }
                Task::none()
            }
            Message::CopyTrigger => {
                let mut clipboard = ClipboardContext::new().unwrap();
                if let Some(header) = self.current_header.clone() {
                    log::debug!("Copying {header} to clipboard.");
                    let _ = clipboard.set_contents(header);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let mut iso_2_codes = get_country_codes();
        iso_2_codes.sort_by(|a, b| natord::compare(a, b));

        let mut consist_list = self.consist_options.clone();
        consist_list.sort_by(|a, b| natord::compare(a, b));

        let reference_input: TextInput<'_, Message> = text_input(
            "",
            self.reference.as_str()
        ).on_input(|s| Message::TextInputChanged(StringInput::Reference, s));
        let description_input: TextInput<'_, Message> = text_input(
            "",
            &self.description.as_str()
        ).on_input(|s| Message::TextInputChanged(StringInput::Description, s));
        let max_speed_input: TextInput<'_, Message> = text_input(
            "",
            format!("{}", &self.start_speed).as_str()
        ).on_input(|s| {
            match s.parse() {
                Ok(n) => Message::NumericInputChanged(IntInput::StartSpeed, n),
                Err(_) => Message::NumericInputChanged(IntInput::StartSpeed, 0),
            }
        });
        column![
            Space::new().height(10),
            row![
                Space::new().width(20),
                column![
                    text("Reference").width(Length::FillPortion(2)),
                    Space::new().height(10),
                    reference_input.width(Length::FillPortion(2))
                ],
                column![
                    text("Description").width(Length::FillPortion(7)),
                    Space::new().height(10),
                    description_input.width(Length::FillPortion(7))
                ],
                column![
                    text("Start Speed (km/h)").width(Length::FillPortion(3)),
                    Space::new().height(10),
                    max_speed_input.width(Length::FillPortion(2))
                ],
                Space::new().width(20)
            ].spacing(10),
            row![
                Space::new().width(20),
                column![
                    text("Country").width(Length::FillPortion(5)),
                    Space::new().height(10),
                    pick_list(iso_2_codes, self.country_code.as_ref(), |s|
                        Message::TextInputChanged(StringInput::Country, s)
                    )
                ],
                column![
                    text("Consist").width(Length::FillPortion(5)),
                    Space::new().height(10),
                    pick_list(consist_list, self.consist.as_ref(), |s|
                        Message::TextInputChanged(StringInput::Consist, s)
                    ).width(Length::FillPortion(5))
                ],
                Space::new().width(20)
            ].spacing(10),
            row![
                Space::new().width(20),
                text(self.current_header.clone().unwrap_or(String::new())).width(
                    Length::FillPortion(8)
                ),
                button("Copy")
                    .on_press(Message::CopyTrigger)
                    .width(Length::FillPortion(2))
                    .style(plain_grey),
                Space::new().width(20)
            ].spacing(10)
        ]
            .spacing(10)
            .into()
    }
}

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Error)
        .with_module_level("railosconsist", log::LevelFilter::Info)
        .init()
        .unwrap();

    let settings = iced::Settings {
        default_text_size: Pixels(12.0),
        ..iced::Settings::default()
    };

    let icon: Option<window::Icon> = load_icon();

    let _ = iced
        ::application(FormData::new, FormData::update, FormData::view)
        .window(window::Settings {
            size: Size { width: 750.0, height: 200.0 },
            icon: icon,
            resizable: false,
            ..window::Settings::default()
        })
        .title(|_state: &FormData|
            format!(
                "Railway Operation Simulator Timetable Header Generator v{}",
                env!("CARGO_PKG_VERSION")
            )
        )
        .settings(settings)
        .theme(Theme::Light)
        .run();
}

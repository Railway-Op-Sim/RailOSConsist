// RailOSConsist - Railway Operation Simulator Timetable Header Generator
//
// Copyright (C) 2026 Kristian Zarębski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Main application module: Provides the UI and logic for generating RailOS
// timetable service headers from consist data.

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

/// Embedded asset container for JSON data files.
///
/// Contains consist data organized by country code directories.
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/data"]
#[include = "*.json"]
struct Asset;

/// Embedded asset container for application media files.
///
/// Contains application icons and other image resources.
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/media"]
#[include = "*.png"]
struct Media;

/// Represents the specifications of a railway consist.
///
/// Contains physical and performance characteristics of a consist, deserialized from JSON data.
///
/// # Fields
/// * `max_speed` - Maximum speed in km/h
/// * `mass` - Mass of the consist in tonnes
/// * `brake_force` - Brake force in tonnes
/// * `power` - Power output in kilowatts
#[derive(Deserialize)]
struct Consist {
    max_speed: u16,
    mass: u16,
    brake_force: u16,
    power: u16,
}

/// Application state containing form inputs and consist data.
///
/// Maintains all user input fields and loaded consist information for a given country.
///
/// # Fields
/// * `reference` - Service reference number (e.g., "1A00")
/// * `description` - Service description
/// * `start_speed` - Starting speed in km/h
/// * `consist` - Currently selected consist name
/// * `country_code` - Currently selected country name
/// * `consist_list` - HashMap of available consists for the selected country
/// * `consist_options` - Sorted list of consist names for UI display
/// * `current_header` - Generated RailOS timetable header string
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

/// Enumeration of string input field types.
///
/// Used to identify which text input field was changed in the UI.
#[derive(Debug, Clone, Copy)]
enum StringInput {
    /// Service reference input
    Reference,
    /// Service description input
    Description,
    /// Country selection input
    Country,
    /// Consist selection input
    Consist,
}

/// Enumeration of integer input field types.
///
/// Used to identify which numeric input field was changed in the UI.
#[derive(Debug, Clone, Copy)]
enum IntInput {
    /// Starting speed input
    StartSpeed,
}

/// Application message types for the Iced event loop.
///
/// Represents all possible user actions and events that can occur in the application.
#[derive(Debug, Clone)]
enum Message {
    /// Text input field value changed
    TextInputChanged(StringInput, String),
    /// Numeric input field value changed
    NumericInputChanged(IntInput, u16),
    /// Copy button pressed
    CopyTrigger,
}

/// Applies a custom grey button theme for the Copy button.
///
/// Provides visual styling that matches the application's UI design with hover and pressed states.
///
/// # Arguments
/// * `_theme` - The current application theme (unused)
/// * `_status` - The button's current interaction status
///
/// # Returns
/// A `button::Style` with grey color scheme and state-dependent shading
fn plain_grey_button_theme(_theme: &Theme, _status: button::Status) -> button::Style {
    // Define the default button style (light grey background)
    let active = button::Style {
        background: Some(Background::Color(Color::from_rgb8(240, 240, 240))),
        text_color: Color::BLACK,
        shadow: Shadow::default(),
        ..button::Style::default()
    };

    // Apply darker shades for hover and pressed states to provide visual feedback
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

/// Loads consist data for a specific country from embedded JSON files.
///
/// Deserializes all JSON files in the country's data directory and merges them into a single HashMap.
///
/// # Arguments
/// * `country_code` - The ISO 3166-1 alpha-2 country code (lowercase)
///
/// # Returns
/// A HashMap mapping consist names to their specifications. Returns empty map if no data found.
///
/// # Panics
/// Logs warnings if JSON parsing fails for individual files.
fn load_data(country_code: String) -> HashMap<String, Consist> {
    log::debug!("Loading country data for {country_code}");
    let mut consist_data: HashMap<String, Consist> = HashMap::new();

    // Construct the directory prefix: country code files are in 'data/<country_code>/'
    let prefix = format!("{}/", country_code);

    // Iterate through all embedded files, filtering for country-specific JSON data
    for file_path in Asset::iter() {
        // Only process files in the country's directory that are JSON files
        if file_path.starts_with(&prefix) && file_path.ends_with(".json") {
            // Extract the file from the embedded assets
            if let Some(embedded_file) = Asset::get(file_path.as_ref()) {
                // Parse the JSON file into a HashMap of consist definitions
                let file_consists: Result<HashMap<String, Consist>, _> = serde_json::from_slice(
                    &embedded_file.data
                );

                // Merge parsed consists into the main HashMap, or log errors
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

/// Retrieves all available country names from embedded data files.
///
/// Extracts country codes from directory structure and converts them to full country names
/// using the ISO 3166-1 standard.
///
/// # Returns
/// A vector of country names that have available consist data, in arbitrary order
fn get_country_codes() -> Vec<String> {
    let mut country_codes: HashSet<String> = HashSet::new();

    for file in Asset::iter() {
        let path = Path::new(file.as_ref());

        // Extract country code from directory structure: data/<country_code>/file.json
        // Uses option chaining to safely extract and convert the parent directory name to a country
        if
            let Some(country_name) = path
                .parent() // Get parent directory (country code)
                .and_then(|p| p.file_name()) // Extract directory name as OsStr
                .and_then(|f| f.to_str()) // Convert to string slice
                .and_then(|n| from_alpha2(n.to_uppercase().as_str())) // Look up ISO country by code
        {
            // Add the full country name (e.g., "Beigium" for "be")
            country_codes.insert(country_name.name.to_string());
        }
    }

    country_codes.into_iter().collect()
}

/// Loads the application icon from embedded media files.
///
/// Attempts to load and parse the RailOSConsist.png image file from embedded resources
/// and convert it to a window icon format.
///
/// # Returns
/// Some(Icon) if the image was successfully loaded and converted, None otherwise
fn load_icon() -> Option<window::Icon> {
    // Load image file from embedded media assets and handle decompression
    let image = match Media::get("RailOSConsist.png") {
        Some(f) => {
            // Decode PNG data from memory
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

    // Extract dimensions and convert image to raw RGBA bytes for window icon
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8().into_raw(); // Convert to 8-bit RGBA and flatten to bytes

    // Create window icon from raw pixel data - this is required by the window system
    match window::icon::from_rgba(rgba, width, height) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

impl FormData {
    /// Creates a new FormData instance with default values.
    ///
    /// Initializes the form with a default reference number of "1A00" and empty other fields.
    ///
    /// # Returns
    /// A tuple containing the new FormData instance and a no-op Task
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
    /// Generates a RailOS timetable header string from the current form data.
    ///
    /// Formats consist specifications into a semicolon-separated header line as required by RailOS.
    ///
    /// # Returns
    /// Some(String) containing the formatted header if reference, description, and consist are set,
    /// None otherwise
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

    /// Processes user input messages and updates the application state.
    ///
    /// Handles changes to text/numeric inputs, country/consist selection, and the copy button press.
    /// Updates derived state (header generation and consist loading) as needed.
    ///
    /// # Arguments
    /// * `message` - The user action message to process
    ///
    /// # Returns
    /// A no-op Task for async operations
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
                        // Look up the selected country in the ISO 3166-1 standard list
                        if let Some(country) = ALL.iter().find(|c| c.name == value) {
                            self.country_code = Some(country.name.to_string());
                            self.consist = None; // Clear previous consist selection
                            // Load consist data for the selected country using its 2-letter code
                            self.consist_list = load_data(country.alpha2.to_lowercase());
                            // Extract consist names from the loaded data for the dropdown menu
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

    /// Builds the user interface layout.
    ///
    /// Creates the form UI with input fields for reference, description, start speed,
    /// country and consist selection dropdowns, header display, and copy button.
    ///
    /// # Returns
    /// An Iced Element representing the complete application UI
    fn view(&self) -> iced::Element<'_, Message> {
        // Sort country codes
        let mut iso_2_codes = get_country_codes();
        iso_2_codes.sort_by(|a, b| natord::compare(a, b));

        // Retrieve and sort consist names using natural order (handles numeric sequences properly)
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
        // Speed input field with automatic parsing from string to integer
        let max_speed_input: TextInput<'_, Message> = text_input(
            "",
            format!("{}", &self.start_speed).as_str()
        ).on_input(|s| {
            // Parse input as integer; default to 0 if parsing fails (non-numeric input)
            match s.parse() {
                Ok(n) => Message::NumericInputChanged(IntInput::StartSpeed, n),
                Err(_) => Message::NumericInputChanged(IntInput::StartSpeed, 0),
            }
        });
        // Build the UI layout using Iced's declarative macros (column, row, etc.)
        // FillPortion divides available space proportionally
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
            // Dropdown selection row for country and consist
            // pick_list: second param is currently selected value, third is message callback
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
                    .style(plain_grey_button_theme),
                Space::new().width(20)
            ].spacing(10)
        ]
            .spacing(10)
            .into()
    }
}

/// Application entry point.
///
/// Initializes logging, configures Iced application settings, loads the window icon,
/// and starts the event loop with FormData as the state model.
fn main() {
    // Configure logging: suppress most debug output but show app-specific info messages
    SimpleLogger::new()
        .with_level(log::LevelFilter::Error) // Default: only show errors
        .with_module_level("railosconsist", log::LevelFilter::Info) // Override for this app
        .init()
        .unwrap();

    // Configure Iced GUI framework settings
    let settings = iced::Settings {
        default_text_size: Pixels(12.0),
        ..iced::Settings::default()
    };

    // Load application icon (returns None if loading fails, UI will render without icon)
    let icon: Option<window::Icon> = load_icon();

    // Initialize the Iced application with three required callbacks:
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

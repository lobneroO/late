// (C) Tim Lobner

use std::process::{Command, Stdio};
use iced::widget::{center, column, row, combo_box, text, pick_list, button};
use iced::{Element, Theme};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;

static CONFIG_NAME: &str = "late_config.json";

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    UpdateBufferSize(u32),
    UpdateSampleRate(u32),
    SaveCurrentSettingsAsProfile,
    LoadProfile, // TODO: needs to be easily choosable in combobox
}

/// Extra state which copies LateState::buffer_size and LateState::sample_rate
/// in order to easily serialize and deserialize them.
/// Serialization is meant for profiles a user may create.
/// E.g: A recording profile (with low latency) and a mixing / everyday profile
/// (with moderate latency allowing for larger buffer sizes)
#[derive(Serialize, Deserialize)]
struct PipewireForceState {
    buffer_size: u32,
    sample_rate: u32
}

/// The LateState is the state of the GUI. It encompasses the current buffer size
/// and sampling rate, as well as the theme and all possible options
struct LateState {
    theme: Theme,

    buffer_sizes: combo_box::State<u32>,
    buffer_size: Option<u32>,
    // the text displayed when a buffer size is selected
    bs_text: String,
    // sample_rates: combo_box::State<SampleRate>,
    sample_rates: combo_box::State<u32>,
    sample_rate: Option<u32>,
    // the text displayed when a sample rate is selected
    sr_text: String,
}

fn get_available_buffer_sizes() -> Vec<u32> {
    vec![
        0, // acts as a reset
        64,
        128,
        256,
        512,
        1024,
        2048,
    ]
}

fn get_available_sample_rates() -> Vec<u32> {
    vec![
        0, // acts as a reset
        22050,
        24000,
        44100,
        48000,
        88200,
        96000,
    ]
}

fn get_current_sample_rate() -> Option<u32> {
    // fetch the current sample rate by terminal command
    let cmd_str = "pw-metadata -n settings 0 clock.force-rate";

    let output = Command::new("sh")
        .arg("-c")
        .stdout(Stdio::piped())
        .arg(cmd_str)
        .output()
        .unwrap();

    // the response lookse something like this
    /*
    * Found "settings" metadata 31
    * update: id:0 key:'clock.force-rate' value:'48000' type:''
    */
    let cmd_return_str = String::from_utf8(output.stdout).unwrap();
    // first remove everything until "value:'"
    let sub1 = &cmd_return_str[cmd_return_str.find("value:'").unwrap_or(0)..];
    // now remove the "value:'" itself
    let sub2 = &sub1["value:'".len()..];
    // lastly, remove everything after the (now) first "'",
    // as that concludes the actual value
    let sub3 = &sub2[0..sub2.find("'").unwrap_or(sub2.len())];

    // turn it into the option for the combo box
    let rate = sub3.parse();
    match rate {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

fn get_current_buffer_size() -> Option<u32> {
    // fetch the current sample rate by terminal command
    let cmd_str = "pw-metadata -n settings 0 clock.force-quantum";

    let output = Command::new("sh")
        .arg("-c")
        .stdout(Stdio::piped())
        .arg(cmd_str)
        .output()
        .unwrap();

    // the response lookse something like this
    /*
    * Found "settings" metadata 31
    * update: id:0 key:'clock.force-quantum' value:'128' type:''
    */
    let cmd_return_str = String::from_utf8(output.stdout).unwrap();
    // first remove everything until "value:'"
    let sub1 = &cmd_return_str[cmd_return_str.find("value:'").unwrap_or(0)..];
    // now remove the "value:'" itself
    let sub2 = &sub1["value:'".len()..];
    // lastly, remove everything after the (now) first "'",
    // as that concludes the actual value
    let sub3 = &sub2[0..sub2.find("'").unwrap_or(sub2.len())];

    // turn it into the option for the combo box
    let rate = sub3.parse();
    match rate {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

impl LateState {

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            buffer_sizes: combo_box::State::new(get_available_buffer_sizes()),
            buffer_size: get_current_buffer_size(),
            bs_text: String::new(),
            sample_rates: combo_box::State::new(get_available_sample_rates()),
            sample_rate: get_current_sample_rate(),
            sr_text: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::UpdateBufferSize(buf_size) => {
                self.buffer_size = Some(buf_size);
                self.bs_text = 
                    buf_size.to_string()
                    + " (" 
                    + &self.latency_as_str()
                    + "ms)";

                // actually execute the change
                let cmd = format!("pw-metadata -n settings 0 clock.force-quantum {}", buf_size);

                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .expect("");
            }
            Message::UpdateSampleRate(rate) => {
                self.sample_rate = Some(rate);
                self.sr_text = 
                    rate.to_string()
                    + " Hz";

                // actually execute the change
                let cmd = format!("pw-metadata -n settings 0 clock.force-rate {}", rate);

                let result = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output();
                match result {
                    Ok(_) => println!("sample rate was set successfully!"),
                    Err(e) => println!("error setting sample rate: {e}"),
                }
            }
            Message::SaveCurrentSettingsAsProfile => {
                save_state(PipewireForceState{
                    sample_rate: self.sample_rate.unwrap_or(0),
                    buffer_size: self.buffer_size.unwrap_or(0),
                });
                print!("Saving!");
            } 
            Message::LoadProfile => {
                print!("Loading");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let buf_size_cbox = combo_box(
            &self.buffer_sizes,
            "Choose a buffer size",
            self.buffer_size.as_ref(),
            Message::UpdateBufferSize,
        );
        let sample_rate_cbox = combo_box(
            &self.sample_rates,
            "Choose a sample rate",
            self.sample_rate.as_ref(),
            Message::UpdateSampleRate,
        );
        let content = column![
            row![
                column![
                    text("Theme:"),
                    pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
                ]
            ]
            .spacing(20),
            row![
                column![
                    text("Buffer Size (Latency):"),
                    buf_size_cbox,
                ],
                column![
                    text("Sample Rate"),
                    sample_rate_cbox,
                ],
            ]
            .spacing(20),
            row![
                button("Save Profile").on_press(Message::SaveCurrentSettingsAsProfile),
                button("Placeholder: Load").on_press(Message::LoadProfile)
            ].spacing(20)
        ]
        .spacing(20)
        .padding(20)
        .max_width(450);

        center(content).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

impl Default for LateState {
    fn default() -> Self {
        LateState::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBufferSizeError(String);
impl ParseBufferSizeError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_owned())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRateError(String);
impl ParseRateError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_owned())
    }
}

impl LateState{
    /// @returns latency in milliseconds as a String
    fn latency_as_str(&self) -> String {
        let l = self.latency();
        l.to_string()
    }

    /// @returns latency in milliseconds
    fn latency(&self) -> f32 {
        if self.buffer_size.is_some() && self.sample_rate.is_some() {
            let buf_size = self.buffer_size.unwrap() as f32;
            let sample_rate = self.sample_rate.unwrap() as f32;
            buf_size * 1000.0 / sample_rate
        }
        else {
            0.0
        }
    }
}

fn ensure_config_file() -> std::io::Result<PathBuf> {
    let home_opt = home::home_dir();
    if home_opt.is_some() {
        let mut config = home_opt.unwrap();
        config.push(".config/late");

        // ensure we can fetch the config dir and exists state
        let config_dir_exists = fs::exists(&config);
        if config_dir_exists.is_err() {
            return Err(config_dir_exists.err().unwrap());
        }

        // ensure we have a config dir 
        if !fs::exists(&config).unwrap() {
            let result = fs::create_dir(&config);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        // ensure we can fetch the config file and its exists state
        config.push(CONFIG_NAME);

        let config_file_exists = fs::exists(&config);
        if config_file_exists.is_err() {
            return Err(config_file_exists.err().unwrap());
        }

        // ensure we have a config file
        if !fs::exists(&config).unwrap() {
            let result = File::create(&config);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        return Ok(config);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Cannot find home directory!"))


}

fn save_state(state: PipewireForceState) {
    let serialized = serde_json::to_string(&state);
    let config_file = match ensure_config_file(){
        Ok(c) => c,
        Err(e) => { 
            print!("{}", e);
            return;
        }
    };

    let f = File::create(config_file);
    write!(f.unwrap(), "{}", serialized.unwrap());

}

fn main() -> iced::Result {
    let icon = iced::window::icon::from_file("resources/late.ico");
    let ico_opt: Option<iced::window::Icon> = icon.ok();
    let win_settings = iced::window::Settings {
        size: iced::Size::new(480.0, 200.0),
        position: iced::window::Position::Default,
        min_size: None,
        max_size: None,
        visible: true,
        resizable: true,
        decorations: true,
        transparent: false,
        level: iced::window::Level::Normal,
        icon: ico_opt,
        platform_specific: iced::window::settings::PlatformSpecific { application_id: "Late".to_owned(), override_redirect: false },
        exit_on_close_request: true,
    };

    iced::application("Late - Pipewire Preferences", LateState::update, LateState::view)
        .theme(LateState::theme)
        .window_size(iced::Size::new(480.0, 230.0))
        .window(win_settings)
        .run()
}

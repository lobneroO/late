// (C) Tim Lobner

use std::process::{Command, Stdio};
use iced::widget::{center, column, row, combo_box, text, pick_list, text_input, button};
use iced::{Element, Theme};

mod profile;
use profile::LateProfile;


#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    UpdateBufferSize(u32),
    UpdateSampleRate(u32),
    SaveProfile,
    DeleteProfile,
    UpdateProfile(String),
    UpdateProfileSaveName(String),
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
    /// name of the current profile, if any
    profile: Option<String>,
    profiles_names: combo_box::State<String>,
    profiles: Vec<LateProfile>,
    profile_save_name: String,
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
    sub3.parse().ok()
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
    sub3.parse().ok()
}

impl LateState {

    fn new(profiles: Vec<LateProfile>) -> Self {
        Self {
            theme: Theme::Dark,
            buffer_sizes: combo_box::State::new(get_available_buffer_sizes()),
            buffer_size: get_current_buffer_size(),
            bs_text: String::new(),
            sample_rates: combo_box::State::new(get_available_sample_rates()),
            sample_rate: get_current_sample_rate(),
            sr_text: String::new(),
            profiles_names: combo_box::State::new(profile::get_profile_names(&profiles)),
            profile: profile::get_current_if_any(&profiles, get_current_sample_rate(), get_current_buffer_size()), //Some("".to_string()),
            profiles,
            profile_save_name: "".to_string(),
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
            Message::UpdateProfile(pro) => {
                let chosen = profile::choose_profile(&self.profiles, &pro);
                if chosen.is_some() {
                    let profile = chosen.unwrap();
                    self.update(Message::UpdateSampleRate(profile.sample_rate));
                    self.update(Message::UpdateBufferSize(profile.buffer_size));
                    self.profile = Some(profile.name.clone());
                } else if pro.is_empty() {
                    // most likely a delete has happened
                    self.profile = None;
                }
            }
            Message::DeleteProfile => {
                let profile_name = self.profile.clone().unwrap();
                profile::remove_profile(&mut self.profiles, &profile_name);
                self.profiles_names = combo_box::State::new(profile::get_profile_names(&self.profiles));
                // saving writes the entire file new. since the profile is deleted from the vector,
                // we save here in order to get it out of the profiles file
                profile::save_profiles(&self.profiles);
                // finally set the profile to empty, since the previously deleted profile must not
                // be enabled anymore, but we have no better guess of what to choose (and we don't
                // want to change the profile here)
                self.update(Message::UpdateProfile("".to_string()));
            }
            Message::SaveProfile => {
                let new_profile = LateProfile {
                    name: self.profile_save_name.clone(),
                    sample_rate: self.sample_rate.unwrap_or(0),
                    buffer_size: self.buffer_size.unwrap_or(0),
                };
                self.profiles.push(new_profile);

                self.profiles_names = combo_box::State::new(profile::get_profile_names(&self.profiles));
                profile::save_profiles(&self.profiles);
                print!("Saving!");
            } 
            Message::UpdateProfileSaveName(pro) => {
                self.profile_save_name = pro;
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
        let profile_name_input = text_input("Profile Name", &self.profile_save_name)
            .on_input(Message::UpdateProfileSaveName)
            .on_submit(Message::SaveProfile);
        let profile_cbox = combo_box(
            &self.profiles_names,
            "Profile",
            self.profile.as_ref(),
            Message::UpdateProfile,
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
                profile_cbox,
                button("Delete Profile").on_press(Message::DeleteProfile),
            ].spacing(20),
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
                profile_name_input,
                button("Save Profile").on_press(Message::SaveProfile),
            ].spacing(20),
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
        LateState::new(profile::load_profiles())
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
        .window_size(iced::Size::new(480.0, 280.0))
        .window(win_settings)
        .run()
}

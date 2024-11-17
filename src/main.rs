
// use std::fmt;
use std::process::{Command, Stdio};
use iced::widget::{column, row, combo_box, text};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    UpdateBufferSize(u32),
    UpdateSampleRate(u32),
}

struct Settings {
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

impl Settings {

    fn new() -> Self {
        Self {
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
        row![
            column![
                text("Buffer Size (Latency):").size(20),
                buf_size_cbox,
            ],
            column![
                text("Sample Rate").size(20),
                sample_rate_cbox,
            ],
        ].into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new()
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

impl Settings{
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
    iced::run("Late - Pipewire Preferences", Settings::update, Settings::view)
}

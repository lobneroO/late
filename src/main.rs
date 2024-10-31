
use std::fmt;
use std::process::{Command, Stdio};
use iced::widget::{column, row, combo_box, text};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    UpdateBufferSize(BufferSize),
    UpdateSampleRate(SampleRate),
}

struct Settings {
    buffer_sizes: combo_box::State<BufferSize>,
    buffer_size: Option<BufferSize>,
    // the text displayed when a buffer size is selected
    bs_text: String,
    sample_rates: combo_box::State<SampleRate>,
    sample_rate: Option<SampleRate>,
    // the text displayed when a sample rate is selected
    sr_text: String,
}

fn get_current_sample_rate() -> Option<SampleRate> {
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

fn get_current_buffer_size() -> Option<BufferSize> {
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
            buffer_sizes: combo_box::State::new(BufferSize::ALL.to_vec()),
            buffer_size: get_current_buffer_size(),
            bs_text: String::new(),
            sample_rates: combo_box::State::new(SampleRate::ALL.to_vec()),
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
            ]
        ]
            .into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BufferSize {
    Buf64,
    Buf128,
    Buf256,
    Buf512,
    #[default]
    Buf1024,
    Buf2048,
}

impl BufferSize {
    const ALL: [BufferSize; 6] = [
        BufferSize::Buf64,
        BufferSize::Buf128,
        BufferSize::Buf256,
        BufferSize::Buf512,
        BufferSize::Buf1024,
        BufferSize::Buf2048,
    ];

    pub fn as_uint(&self) -> u32{
        match self {
            // There's probably a nicer way to do this, but it is late, I can't be bothered
            BufferSize::Buf64 => 64,
            BufferSize::Buf128 => 128,
            BufferSize::Buf256 => 256,
            BufferSize::Buf512 => 512,
            BufferSize::Buf1024 => 1024,
            BufferSize::Buf2048 => 2048,
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBufferSizeError(String);
impl ParseBufferSizeError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_owned())
    }
}

impl std::str::FromStr for BufferSize {
    type Err = ParseRateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This is the worst code I have written in a while :)
        if s == "64" {
            Ok(BufferSize::Buf64)
        }
        else if s == "128" {
            Ok(BufferSize::Buf128)
        }
        else if s == "256" {
            Ok(BufferSize::Buf256)
        }
        else if s == "512" {
            Ok(BufferSize::Buf512)
        }
        else if s == "1024" {
            Ok(BufferSize::Buf1024)
        }
        else if s == "2048" {
            Ok(BufferSize::Buf2048)
        }
        else {
            Err(ParseRateError::new("Invalid sample rate!"))
        }
    }
}

impl fmt::Display for BufferSize {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_uint().to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SampleRate {
    Rate22050,
    Rate24000,
    Rate44100,
    #[default]
    Rate48000,
    Rate96000,
}

impl SampleRate {
    const ALL: [SampleRate; 5] = [
        SampleRate::Rate22050,
        SampleRate::Rate24000,
        SampleRate::Rate44100,
        SampleRate::Rate48000,
        SampleRate::Rate96000,
    ];

    pub fn as_uint(&self) -> u32 {
        match self {
            SampleRate::Rate22050 => 22050,
            SampleRate::Rate24000 => 24000,
            SampleRate::Rate44100 => 44100,
            SampleRate::Rate48000 => 48000,
            SampleRate::Rate96000 => 96000,
        }
    }
}

impl fmt::Display for SampleRate {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_uint().to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRateError(String);
impl ParseRateError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_owned())
    }
}

impl std::str::FromStr for SampleRate {
    type Err = ParseRateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This is the worst code I have written in a while :)
        if s == "22050" {
            Ok(SampleRate::Rate22050)
        }
        else if s == "24000" {
            Ok(SampleRate::Rate24000)
        }
        else if s == "44100" {
            Ok(SampleRate::Rate44100)
        }
        else if s == "48000" {
            Ok(SampleRate::Rate48000)
        }
        else if s == "96000" {
            Ok(SampleRate::Rate96000)
        }
        else {
            Err(ParseRateError::new("Invalid sample rate!"))
        }
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
            let buf_size = self.buffer_size.unwrap().as_uint() as f32;
            let sample_rate = self.sample_rate.unwrap().as_uint() as f32;
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

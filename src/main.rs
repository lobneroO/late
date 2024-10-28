
use std::fmt;
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

impl Settings {

    fn new() -> Self {
        Self {
            // TODO: should query the current buffer size and sample rate
            buffer_sizes: combo_box::State::new(BufferSize::ALL.to_vec()),
            buffer_size: None,
            bs_text: String::new(),
            sample_rates: combo_box::State::new(SampleRate::ALL.to_vec()),
            sample_rate: None,
            sr_text: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        // TODO:
        match message {
            Message::UpdateBufferSize(buf_size) => {
                self.buffer_size = Some(buf_size);
                self.bs_text = 
                    buf_size.to_string()
                    + " (" 
                    + &self.latency_as_str()
                    + "ms)";
            }
            Message::UpdateSampleRate(rate) => {
                self.sample_rate = Some(rate);
                self.sr_text = 
                    rate.to_string()
                    + " Hz";
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

impl Settings{
    /// @returns latency in milliseconds as a String
    fn latency_as_str(&self) -> String {
        let l = self.latency();
        l.to_string()
    }

    /// @returns latency in milliseconds
    fn latency(&self) -> f32 {
        if self.buffer_size != None && self.sample_rate != None {
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

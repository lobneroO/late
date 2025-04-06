
use std::process::{Command, Stdio};

pub fn get_available_buffer_sizes() -> Vec<u32> {
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

pub fn get_buffer_size_value(buffer_size_with_info: &str) -> u32 {
    // the buffer size info will have a latency added if possible.
    // e.g. 128 (2.4ms)
    // therefore find the first space and discard everything afterwards.
    // if the sample rate is not set, then ther is no latency info, 
    // e.g. 128
    // so nothing needs to be discarded
    buffer_size_with_info[..buffer_size_with_info.len()-3].parse::<u32>().unwrap()
}

pub fn get_buffer_size_string(rate: u32) -> String {
    format!("{} (x ms)", rate)
}

pub fn get_available_buffer_sizes_with_latency() -> Vec<String> {
    let sizes = get_available_buffer_sizes();
    let mut sizes_latency = Vec::new();

    for size in sizes {
        sizes_latency.push(get_buffer_size_string(size));
    }

    sizes_latency
}

pub fn get_current_buffer_size() -> Option<u32> {
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

pub fn get_current_buffer_size_with_latency() -> Option<String> {
    let size = get_current_buffer_size();
    match size {
        Some(s) => Some(get_buffer_size_string(s)),
        None => None
    }
}

pub fn set_buffer_size(size: u32) {
    let cmd = format!("pw-metadata -n settings 0 clock.force-quantum {}", size);

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("");
}

/// Sets the buffer size with a string that (possibly) contains latency
/// (this will not set the buffer size to achieve a certain latency!)
pub fn set_buffer_size_with_latency(size: &str) {
    set_buffer_size(get_buffer_size_value(size));
}

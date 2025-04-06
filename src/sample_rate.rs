
use std::process::{Command, Stdio};

// this file does all the sample rate handling
// sample rate will be given in Hz, and displayed with the unit
// e.g.: 
//      0 Hz
//      22050 Hz
//      24000 Hz
//      etc
//  However, the profile value will be a u32, which makes
//  manually editing the json file a lot less error prone
//
//  for converting from u32 to String, use get_sample_rate_string,
//  for converting from String to u32, use get_sample_rate_value

pub fn get_available_sample_rates() -> Vec<u32> {
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

pub fn get_sample_rate_value(rate_with_unit: &str) -> u32 {
    rate_with_unit[..rate_with_unit.len()-3].parse::<u32>().unwrap()
}

pub fn get_sample_rate_string(rate: u32) -> String {
    format!("{} Hz", rate)
}

pub fn get_sample_rates_with_units() -> Vec<String> {
    let rates = get_available_sample_rates();
    let mut rates_units = Vec::new();

    for rate in rates {
        rates_units.push(get_sample_rate_string(rate));
    }

    rates_units
}

pub fn get_current_sample_rate() -> Option<u32> {
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

pub fn get_current_sample_rate_with_units() -> Option<String> {
    let rate = get_current_sample_rate();
    match rate {
        Some(r) => Some(get_sample_rate_string(r)),
        None => None
    }
}

pub fn set_sample_rate(rate: u32) {
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

pub fn set_sample_rate_with_units(rate: &str) {
    set_sample_rate(get_sample_rate_value(rate));
}


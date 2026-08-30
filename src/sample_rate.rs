
use std::process::{Command, Stdio};

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

pub fn get_current_sample_rate() -> Option<u32> {
    // fetch the current sample rate by terminal command
    let prgrm_call = "pw-metadata";
    let cmd_args = ["-n", "settings", "0", "clock.force-rate"];

    let output = if std::path::Path::new("/.flatpak-info").exists() {
        println!("using flatpak-spawn --host");
        Command::new("flatpak-spawn")
            .arg("--host")
            .arg(prgrm_call)
            .args(cmd_args)
            .stdout(Stdio::piped())
            .output()
            .unwrap()
    } else {
        println!("using sh -c");
        Command::new("sh")
            .arg("-c")
            .stdout(Stdio::piped())
            .arg(prgrm_call)
            .args(cmd_args)
            .output()
            .unwrap()
    };

    // the response lookse something like this
    /*
    * Found "settings" metadata 31
    * update: id:0 key:'clock.force-rate' value:'48000' type:''
    */
    let cmd_return_str = String::from_utf8(output.stdout).unwrap();
    println!("{}", cmd_return_str);
    // first remove everything until "value:'"
    let sub1 = &cmd_return_str[cmd_return_str.find("value:'").unwrap_or(0)..];
    println!("{}", sub1);
    // now remove the "value:'" itself
    let sub2 = &sub1["value:'".len()..];
    // lastly, remove everything after the (now) first "'",
    // as that concludes the actual value
    let sub3 = &sub2[0..sub2.find("'").unwrap_or(sub2.len())];

    // turn it into the option for the combo box
    sub3.parse().ok()
}

pub fn set_sample_rate(rate: u32) {

    // actually execute the change
    let cmd = format!("pw-metadata -n settings 0 clock.force-rate {}", rate);

    let result = if std::path::Path::new("/.flatpak-info").exists() {
        Command::new("flatpak-spawn")
            .arg("--host")
            .arg(cmd)
            .stdout(Stdio::piped())
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .stdout(Stdio::piped())
            .arg(cmd)
            .output()
    };

    match result {
        Ok(_) => println!("sample rate was set successfully!"),
        Err(e) => println!("error setting sample rate: {e}"),
    }
}


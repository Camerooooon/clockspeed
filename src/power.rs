use std::fmt;
use super::Error;
use std::fs::File;
use std::path::Path;
use std::io::{Read};

pub enum LidState {
    Open,
    Closed,
    Unapplicable,
    Unknown,
}

impl fmt::Display for LidState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            LidState::Open => write!(f, "open"),
            LidState::Closed => write!(f, "closed"),
            LidState::Unapplicable => write!(f, "unapplicable"),
            LidState::Unknown => write!(f, "unknown"),
       }
    }
}

pub fn read_lid_state() -> Result<LidState, Error> {
    if !Path::new("/proc/acpi/button/lid/LID0/state").exists() {
        return Ok(LidState::Unapplicable);
    }

    let mut lid_str: String = String::new();
    File::open("/proc/acpi/button/lid/LID0/state")?.read_to_string(&mut lid_str)?;
    
    if lid_str.contains("open") {
        return Ok(LidState::Open)
    } else if lid_str.contains("closed") {
        return Ok(LidState::Closed)
    }

    Ok(LidState::Unknown)
}

pub fn read_battery_charge() -> Result<i8, Error> {
    if !Path::new("/sys/class/power_supply/BAT0/capacity").exists() {
        return Ok(100);
    }

    let mut cap_str: String = String::new();
    File::open("/sys/class/power_supply/BAT0/capacity")?.read_to_string(&mut cap_str)?;
   
    // Remove the \n char
    cap_str.pop();

    Ok(cap_str.parse::<i8>().unwrap())
}

pub fn read_power_source() -> Result<bool, Error> {
    if !Path::new("/sys/class/power_supply/AC0/online").exists() {
        println!("Unexpected, the directory /sys/class/power_supple/AC0/online doesn't exist? Do you not have a power source?");
        return Ok(true);
    }

    let mut pwr_str: String = String::new();
    File::open("/sys/class/power_supply/AC0/online")?.read_to_string(&mut pwr_str)?;
   
    // Remove the \n char
    pwr_str.pop();

    return Ok(pwr_str == "1");
}
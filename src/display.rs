use super::cpu::CPU;
use super::power::LidState;
use std::fmt::Display;
use termion::{color, style};

pub fn print_freq(f: i32, raw: bool) {
    if raw {
        println!("{}", f);
    } else {
        println!("CPU freq is {} MHz", f / 1000)
    }
}

pub fn print_power(lid: LidState, bat: i8, plugged: bool, raw: bool) {
    if raw {
        println!("{} {} {}", lid, bat, plugged);
    } else {
        println!("Lid: {} Battery: {} Plugged: {}", lid, bat, plugged);
    }
}

pub fn print_turbo(t: bool, raw: bool) {
    if raw {
        println!("{}", t);
    } else {
        println!(
            "{}",
            if t {
                "Turbo is enabled"
            } else {
                "Turbo is not enabled"
            }
        )
    }
}

fn print_vec<T: Display>(t: Vec<T>, raw: bool) {
    if raw {
        for x in t {
            println!("{}", x);
        }
    } else {
        for x in t {
            print!("{} ", x);
        }
        print!("\n")
    }
}

pub fn print_available_governors(available_governors: Vec<String>, raw: bool) {
    print_vec(available_governors, raw);
}

pub fn print_cpus(cpus: Vec<CPU>, name: String, raw: bool) {
    if raw {
        for x in cpus {
            println!("{} {}", x.name, smooth_i32_vec(x.cur_freq, 3));
        }
    } else {
        println!("Name:{}", name);
        for x in cpus {
            println!("{} is currently @ {} MHz", x.name, smooth_i32_vec(x.cur_freq, 3) / 1000);
        }
    }
}

pub fn print_cpu(cpu: &CPU) {
    let mut temp_color: String = color::Fg(color::Green).to_string();

    if cpu.cur_temp / 1000 > 60 {
        temp_color = color::Fg(color::Red).to_string();
    } else if cpu.cur_temp / 1000 > 40 {
        temp_color = color::Fg(color::Yellow).to_string();
    }

    let cur_freq = &cpu.cur_freq;

    println!(
        "{}{}:{} {}Hz\t{}Hz\t{}{}Hz{}\t{}C{}\t{}",
        style::Bold,
        cpu.name,
        style::Reset,
        cpu.max_freq / 1000,
        cpu.min_freq / 1000,
        color::Fg(color::Green),
        smooth_i32_vec(cur_freq.to_vec(), 3) / 1000,
        temp_color,
        cpu.cur_temp / 1000,
        style::Reset,
        cpu.gov
    );
}

pub fn print_cpu_speeds(cpu_speeds: Vec<i32>, raw: bool) {
    print_vec(cpu_speeds, raw);
}

pub fn print_cpu_temp(cpu_temp: Vec<i32>, raw: bool) {
    print_vec(cpu_temp, raw);
}

pub fn print_cpu_governors(cpu_governors: Vec<String>, raw: bool) {
    print_vec(cpu_governors, raw);
}

pub fn smooth_i32_vec(smooth: Vec<i32>, iterations: i8) -> i32 {

    let mut total = 0;
    let mut count = 0;
    for i in 0..iterations {
        let value = smooth.get(i as usize);
        if value == None {
            break;
        }

        total = total + *value.unwrap();
        count = count + 1;
    }
    return total/count;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acs_smooth_test() {
        let values = vec![69, 42, 10, 24, 124, 300, 1000000];
        assert_eq!(smooth_i32_vec(values, 4), 36);
    }
}

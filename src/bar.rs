use crate::languages::*;
use color_please::{reset_fg, set_fg, Color};
use std::collections::HashMap;
use std::process;
use terminal_size::{terminal_size, Height, Width};

pub fn bar(languages_usage: &HashMap<Language, u128>) {
    let width = match terminal_size() {
        Some((Width(w), Height(_))) => w,
        None => {
            eprintln!("Unable to get terminal size!");
            process::exit(1);
        }
    };
    let languages_usage_iter = languages_usage.iter();
    let mut total_lines: u128 = 0;
    for (_, amount) in languages_usage_iter.clone() {
        total_lines += *amount;
    }
    let mut other = 0;
    let mut ratios = Vec::new();
    let mut chars_to_display = Vec::new();
    for (lang, amount) in languages_usage_iter {
        let ratio = *amount as f64 / total_lines as f64;
        let char_count = (width - 1) as f64 * ratio;
        if char_count < 2.0 {
            other += *amount;
            ratios.push((lang, ratio * 100.0));
        } else {
            chars_to_display.push((lang, char_count));
            ratios.push((lang, ratio * 100.0));
        }
    }
    chars_to_display.sort_by(|(_, b), (_, d)| d.partial_cmp(b).unwrap());
    ratios.sort_by(|(_, b), (_, d)| d.partial_cmp(b).unwrap());
    print!("|");
    for (lang, char_count) in chars_to_display.iter() {
        lang.set_color_for();
        for _ in 1..(*char_count as u64) {
            print!("―");
        }
        reset_fg();
    }
    if other > 0 {
        for _ in 1..(((width - 1) as f64 * (other as f64 / total_lines as f64)) as u16) {
            print!("―");
        }
    }
    println!("|");
    for (lang, ratio) in ratios.iter() {
        lang.set_color_for();
        println!("{}: {}%", lang, (ratio * 10.0).round() / 10.0);
        reset_fg();
    }
    set_fg(Color::Color256(240));
    for _ in 1..width {
        print!("―");
    }
    println!();
    reset_fg();
    println!("Total lines of code: {}", total_lines);
}

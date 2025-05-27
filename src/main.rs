// See license for copyright information

use colored::Colorize;
use serde_json::Value;
use std::collections::HashMap;

fn trim_outer_quotes(s: &mut String) -> String {
    s.remove(0);
    s.remove(s.len() - 1);
    s.clone()
}

fn trim_full_stop(s: &mut String) -> String {
    let mut s_iter = s.split(".");
    let s_upper_section = s_iter
        .next()
        .expect("Failed to trim whitespace")
        .to_string();
    s_upper_section.clone()
}

fn print_color_and_desc(v: &Value) {
    let lines_to_colors: HashMap<String, [u8; 3]> = HashMap::from([
        ("Bakerloo".to_string(), [178, 99, 0]),
        ("Central".to_string(), [220, 36, 31]),
        ("Circle".to_string(), [255, 200, 10]),
        ("District".to_string(), [0, 125, 50]),
        ("Hammersmith & City".to_string(), [245, 137, 166]),
        ("Jubilee".to_string(), [131, 141, 147]),
        ("Metropolitan".to_string(), [155, 0, 88]),
        ("Northern".to_string(), [0, 0, 0]),
        ("Piccadilly".to_string(), [0, 25, 168]),
        ("Victoria".to_string(), [3, 155, 229]),
        ("Waterloo & City".to_string(), [118, 208, 189]),
    ]);
    let mut name_of_line = v.get("name").expect("Failed to get name").to_string();
    trim_outer_quotes(&mut name_of_line);
    let rgb: [u8; 3] = lines_to_colors[&name_of_line];
    let line_status: Vec<Value> = serde_json::from_str(
        v.get("lineStatuses")
            .expect("Failed to get line status")
            .to_string()
            .as_str(),
    )
    .expect("Failed to parse line_status");
    let mut status_severity_description = line_status[0]
        .get("statusSeverityDescription")
        .expect("Failed to parse status severity description")
        .to_string();
    trim_outer_quotes(&mut status_severity_description);

    let mut status_reason: String = String::new();
    if status_severity_description != "Good Service" {
        status_reason = line_status[0]
            .get("reason")
            .expect("Failed to parse status reason")
            .to_string();
        trim_outer_quotes(&mut status_reason);
    }
    let good_service = status_severity_description == "Good Service";
    if good_service {
        println!(
            "{}: {}",
            name_of_line.truecolor(rgb[0], rgb[1], rgb[2]),
            status_severity_description
        );
    } else {
        let mut status_reason_iter = status_reason.split(":");
        let _ = status_reason_iter.next();
        let status_reason_trimmed = status_reason_iter
            .next()
            .expect("Failed to parse status reason");
        println!(
            "{}:{}",
            name_of_line.truecolor(rgb[0], rgb[1], rgb[2]),
            status_reason_trimmed.to_string()
        );
    }
}

fn main() {
    let tfl_response: String =
        reqwest::blocking::get("https://api.tfl.gov.uk/line/mode/tube/status")
            .expect("Failed to make the API request")
            .text()
            .expect("Failed to convert API request to text");

    let response: Vec<Value> =
        serde_json::from_str(tfl_response.as_str()).expect("Failed to serialize API response");
    for entry in response {
        print_color_and_desc(&entry);
    }
}

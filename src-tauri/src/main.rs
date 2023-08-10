// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate regex;
use regex::Regex;
use std::fs;

fn ls(path: &str) -> String {
    let mut data = String::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let s = format!("{:?},", path.display());
            data.push_str(&s);
        } else {
            let s = format!("{:?},", path.display());
            data.push_str(&s);
        }
    }
    data
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn parse_command(command: &str) -> String {
    if command == "ls" {
        format!("{}", ls("./"))
    } else {
        format!("yuck, not a good command")
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

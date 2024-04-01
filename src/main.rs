#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Board Checker",
        options,
        Box::new(|_cc| {
            Box::<BoardPhrase>::default()
        }),
    )
}

struct BoardPhrase {
    phrase: String,
    invalid_label: String,
    valid_label: String,
    char_count: HashMap<char, i32>
}

impl Default for BoardPhrase {
    fn default() -> Self {
        Self {
            phrase: "".to_owned(),
            invalid_label: "Invalid characters:".to_owned(),
            valid_label: "Valid characters:".to_owned(),
            char_count: HashMap::from([
                ('A', 8),
                ('B', 3),
                ('C', 5),
                ('D', 5),
                ('E', 9),
                ('F', 3),
                ('G', 3),
                ('H', 5),
                ('I', 8),
                ('J', 2),
                ('K', 2),
                ('L', 6),
                ('M', 4),
                ('N', 6),
                ('O', 8),
                ('P', 4),
                ('Q', 2),
                ('R', 6),
                ('S', 8),
                ('T', 8),
                ('U', 6),
                ('V', 2),
                ('W', 2),
                ('X', 2),
                ('Y', 4),
                ('Z', 2),
                ('0', 4),
                ('1', 4),
                ('2', 3),
                ('3', 3),
                ('4', 3),
                ('5', 3),
                ('6', 3),
                ('7', 3),
                ('8', 3),
                ('9', 3),
                ('@', 2),
                ('?', 1),
                (',', 3),
                ('(', 1),
                (')', 1),
                ('{', 1),
                ('}', 1),
                ('\'', 1),
                ('.', 3),
                ('&', 1),
                ('!', 1),
                ('*', 1),
                ('+', 1),
                ('~', 1),
                ('#', 1),
                ('$', 1)
            ])
        }
    }
}

impl eframe::App for BoardPhrase {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let r = ui.text_edit_singleline(&mut self.phrase);

            if r.changed() {
                self.invalid_label = "Invalid characters:".to_owned();
                self.valid_label = "Valid characters:".to_owned();

                let mut actual_char_count: HashMap<char, i32> = HashMap::from([]);
                for c in self.phrase.chars() {
                    if c == ' ' {
                        continue;
                    }

                    let c = c.to_ascii_uppercase();
                    *actual_char_count.entry(c).or_insert(0) += 1;
                }

                for kvp in actual_char_count.iter() {
                    if !self.char_count.contains_key(kvp.0) {
                        self.invalid_label.push_str(format!("\n\t{}: {}", kvp.0, kvp.1).as_str());
                    }
                    else if self.char_count[kvp.0] < *kvp.1 {
                        self.invalid_label.push_str(format!("\n\t{}: {}", kvp.0, kvp.1 - self.char_count[kvp.0]).as_str());
                        self.valid_label.push_str(format!("\n\t{}: {}", kvp.0, self.char_count[kvp.0]).as_str());
                    }
                    else {
                        self.valid_label.push_str(format!("\n\t{}: {}", kvp.0, kvp.1).as_str());
                    }
                }
            }

            if self.invalid_label != "Invalid characters:" {
                ui.label(&self.invalid_label);
            }

            ui.label(&self.valid_label);
        });
    }
}
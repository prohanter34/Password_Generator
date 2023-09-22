#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rand::Rng;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(650.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Password generator",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    probability: f64,
    speed: f64,
    time: f64,
    probability_t: String,
    speed_t: String,
    time_t: String,
    low_border: f64,
    power: i64,
    length: i64,
    checkbox: [bool; 7],
    password: String,
    data: [String; 7],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            probability: 0.00001,
            speed: 15.0,
            time: 14.0,
            low_border: 0.0,
            power: 0,
            length: 0,
            probability_t: String::from("0.00001"),
            speed_t: String::from("15"),
            time_t: String::from("14"),
            checkbox: [true, true, false, false, false, false, true],
            password: String::new(),
            data: [String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), String::from("abcdefghijklmnopqrstuvwxyz"), 
                   String::from("АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"), String::from("абвгдеёжзийклмнопрстуфхцчшщъыьэюя"), 
                   String::from("!@#$%-"), String::from("^&*()_+~\"№;%:?=[]{}|/,.'<>"), String::from("1234567890")],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.probability_t);
                            ui.label("P (вероятность)");

                        });
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.speed_t);
                            ui.label("V (скорость перебора в мин)");

                        });
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.time_t);
                            ui.label("T (срок действия пароляв днях)");

                        });
                        ui.horizontal(|ui| {
                            ui.label(self.low_border.to_string());
                            ui.label("S* (нижняя граница паролей)");

                        });
                        ui.horizontal(|ui| {
                            ui.label(self.power.to_string());
                            ui.label("A (мощность алфавита)");

                        });
                        ui.horizontal(|ui| {
                            ui.label(self.length.to_string());
                            ui.label("L (длинна пароля)");
                        });
                    });
                    

                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.checkbox[0], "Латинские большие");
                        ui.checkbox(&mut self.checkbox[1], "Латинские маленькие");
                        ui.checkbox(&mut self.checkbox[2], "Русские большие");
                        ui.checkbox(&mut self.checkbox[3], "Русские маленькие");
                        ui.checkbox(&mut self.checkbox[4], "Символы");
                        ui.checkbox(&mut self.checkbox[5], "Символы доп.");
                        ui.checkbox(&mut self.checkbox[6], "Цифры");
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("ПАРОЛЬ: ");
                    ui.label(&self.password);
                });
                if ui.button("Сформировать пароль").clicked() {
                    self.password = String::new();
                    self.speed = self.speed_t.parse::<f64>().unwrap() * 60.0;
                    self.probability = self.probability_t.parse().unwrap();
                    self.time = self.time_t.parse::<f64>().unwrap() * 24.0;
                    self.low_border = (self.speed * self.time) / self.probability;
                    self.low_border = self.low_border.ceil();
                    let mut alphabet = String::new();
                    for i in 0..7 {
                        if self.checkbox[i] {

                            alphabet += &self.data[i];
                        }
                    }
                    self.power = alphabet.len() as i64;
                    self.length = (self.low_border as f64).log(self.power as f64).ceil() as i64;
                    let alphabet_array: Vec<char> = alphabet.chars().collect();
                    for _i in 0..self.length {
                        self.password.push(alphabet_array[rand::thread_rng().gen_range(0..alphabet_array.len()) as usize]);
                    }
                }
            });
        });
    }
}

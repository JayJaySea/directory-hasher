use eframe::egui; 
use sha_1::{
    hasher::Hasher,
    Lang,
    NoThreads,
    HasherError
};

use std::time::Instant;
use egui_file::FileDialog;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("SHA-1 directory hasher", native_options, Box::new(move |cc| Box::new(ShaGui::new(cc))));
}

struct ShaGui {
    input: String,
    input_dialog: Option<FileDialog>,
    output: String,
    output_dialog: Option<FileDialog>,
    no_threads: NoThreads,
    lang: Lang,
    time: u128,
    message: String
}

impl ShaGui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input: String::new(),
            input_dialog: None,
            output: String::new(),
            output_dialog: None,
            no_threads: NoThreads::One(1),
            lang: Lang::C,
            time: 0,
            message: String::new()
        }
    }
}

impl eframe::App for ShaGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SHA-1");
            ui.horizontal(|ui| {
                if ui.button("Choose input folder").clicked() {
                    let mut dialog = FileDialog::select_folder(Some("\\.".into()));
                    dialog.open();
                    self.input_dialog = Some(dialog);
                };
                ui.label(self.input.clone());
            });

            if let Some(dialog) = &mut self.input_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.input = file.into_os_string().into_string().unwrap();
                    }
                }
            }
            ui.horizontal(|ui| {
                if ui.button("Choose output folder").clicked() {
                    let mut dialog = FileDialog::select_folder(Some("\\.".into()));
                    dialog.open();
                    self.output_dialog = Some(dialog);
                };
                ui.label(self.output.clone());
            });
            if let Some(dialog) = &mut self.output_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.output = file.into_os_string().into_string().unwrap();
                    }
                }
            }


            ui.radio_value(&mut self.lang, Lang::C, "C");
            ui.radio_value(&mut self.lang, Lang::Asm, "Asm");

            let number = self.get_number_of_threads();

            egui::ComboBox::from_label("")
                .selected_text(format!("{} threads", number))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.no_threads, NoThreads::One(1), "1");
                    ui.selectable_value(&mut self.no_threads, NoThreads::Two(2), "2");
                    ui.selectable_value(&mut self.no_threads, NoThreads::Four(4), "4");
                    ui.selectable_value(&mut self.no_threads, NoThreads::Eight(8), "8");
                    ui.selectable_value(&mut self.no_threads, NoThreads::Sixteen(16), "16");
                    ui.selectable_value(&mut self.no_threads, NoThreads::ThirtyTwo(32), "32");
                    ui.selectable_value(&mut self.no_threads, NoThreads::SixtyFour(64), "64");
                }
            );

            if ui.button("Hash!").clicked() {
                let mut hasher = Hasher::new(number, self.lang);

                let now = Instant::now();
                match hasher.hash_files_in_directory(&self.input, &self.output) {
                    Ok(_) => {
                        self.time = now.elapsed().as_micros();
                        self.message = String::new();
                    },
                    Err(HasherError::BadInputLocation) => {
                        self.time = 0;
                        self.message = String::from("Make sure to choose valid input directory!");
                    }
                    Err(HasherError::BadOutputLocation) => {
                        self.time = 0;
                        self.message = String::from("Make sure to choose valid output directory!");
                    }
                    Err(HasherError::AsmLibLoadingError) => {
                        self.time = 0;
                        self.message = String::from("Make sure you put folder with asm and c libraries in folder with executable!");
                    }
                    Err(HasherError::CLibLoadingError) => {
                        self.time = 0;
                        self.message = String::from("Make sure you put folder with asm and c libraries in folder with executable!");
                    }
                }
            };

            ui.label(format!("{} µs", self.time));
            ui.label(self.message.clone());
        });
    }
}

impl ShaGui {
    fn get_number_of_threads(&self) -> u8 {
           match self.no_threads {
               NoThreads::One(x) => x,
               NoThreads::Two(x) => x,
               NoThreads::Four(x) => x,
               NoThreads::Eight(x) => x,
               NoThreads::Sixteen(x) => x,
               NoThreads::ThirtyTwo(x) => x,
               NoThreads::SixtyFour(x) => x
           }
    }
}
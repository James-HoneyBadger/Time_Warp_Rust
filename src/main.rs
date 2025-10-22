use eframe::egui;
use rfd::FileDialog;

struct TimeWarpApp {
    code: String,
    output: String,
    language: String,
    tab: Tab,
    code_history: Vec<String>,
    code_history_index: usize,
    last_file_path: Option<String>,
}

#[derive(PartialEq)]
enum Tab {
    Editor,
    Output,
}

impl Default for TimeWarpApp {
    fn default() -> Self {
        Self {
            code: String::from("REM Type your code here..."),
            output: String::from("Welcome to Time Warp IDE!\n"),
            language: String::from("BASIC"),
            tab: Tab::Editor,
            code_history: vec![String::from("REM Type your code here...")],
            code_history_index: 0,
            last_file_path: None,
        }
    }
}

impl eframe::App for TimeWarpApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Track code changes for Undo/Redo
        if self.code_history.is_empty() || self.code != self.code_history[self.code_history_index] {
            if self.code_history_index + 1 < self.code_history.len() {
                self.code_history.truncate(self.code_history_index + 1);
            }
            self.code_history.push(self.code.clone());
            self.code_history_index = self.code_history.len() - 1;
        }
        // Light theme for a more educational/clean look
        ctx.set_visuals(egui::Visuals::light());

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // === FILE MENU ===
                ui.menu_button("📁 File", |ui| {
                    if ui.button("📄 New File").clicked() {
                        self.code.clear();
                        self.output = "New file created.".to_string();
                        self.code_history = vec![String::new()];
                        self.code_history_index = 0;
                        self.last_file_path = None;
                        ui.close_menu();
                    }
                    if ui.button("📂 Open File...").clicked() {
                        if let Some(path) = FileDialog::new().add_filter("Text", &['txt', "tw", "bas", "logo", "pas", "plg"]).pick_file() {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                self.code = content;
                                self.output = format!("Opened file: {}", path.display());
                                self.code_history = vec![self.code.clone()];
                                self.code_history_index = 0;
                                self.last_file_path = Some(path.display().to_string());
                            } else {
                                self.output = format!("Failed to open file: {}", path.display());
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("💾 Save").clicked() {
                        let mut saved = false;
                        if let Some(path) = &self.last_file_path {
                            if std::fs::write(path, &self.code).is_ok() {
                                self.output = format!("Saved to {}", path);
                                saved = true;
                            }
                        }
                        if !saved {
                            if let Some(path) = FileDialog::new().set_file_name("untitled.tw").save_file() {
                                if std::fs::write(&path, &self.code).is_ok() {
                                    self.output = format!("Saved to {}", path.display());
                                    self.last_file_path = Some(path.display().to_string());
                                } else {
                                    self.output = format!("Failed to save file: {}", path.display());
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("💾 Save As...").clicked() {
                        if let Some(path) = FileDialog::new().set_file_name("untitled.tw").save_file() {
                            if std::fs::write(&path, &self.code).is_ok() {
                                self.output = format!("Saved to {}", path.display());
                                self.last_file_path = Some(path.display().to_string());
                            } else {
                                self.output = format!("Failed to save file: {}", path.display());
                            }
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🚪 Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                // === EDIT MENU ===
                ui.menu_button("✏️ Edit", |ui| {
                    if ui.button("↩️ Undo").clicked() {
                        if self.code_history_index > 0 {
                            self.code_history_index -= 1;
                            self.code = self.code_history[self.code_history_index].clone();
                            self.output = "Undo".to_string();
                        }
                        ui.close_menu();
                    }
                    if ui.button("↪️ Redo").clicked() {
                        if self.code_history_index + 1 < self.code_history.len() {
                            self.code_history_index += 1;
                            self.code = self.code_history[self.code_history_index].clone();
                            self.output = "Redo".to_string();
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("✂️ Cut").clicked() {
                        ctx.output_mut(|o| o.copied_text = self.code.clone());
                        self.code.clear();
                        self.output = "Cut".to_string();
                        ui.close_menu();
                    }
                    if ui.button("📋 Copy").clicked() {
                        ctx.output_mut(|o| o.copied_text = self.code.clone());
                        self.output = "Copy".to_string();
                        ui.close_menu();
                    }
                    if ui.button("📄 Paste").clicked() {
                        let clipboard = ctx.input(|i| i.raw.clipboard.clone().unwrap_or_default());
                        self.code.push_str(&clipboard);
                        self.output = "Paste".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🔍 Find/Replace...").clicked() {
                        self.output = "Find/Replace not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🔍 Find Next").clicked() {
                        self.output = "Find Next not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("💬 Toggle Comment").clicked() {
                        // Simple toggle: add/remove // at line start
                        let mut new_code = String::new();
                        for line in self.code.lines() {
                            if line.trim_start().starts_with("//") {
                                new_code.push_str(line.trim_start_matches("//"));
                            } else {
                                new_code.push_str(&format!("//{}", line));
                            }
                            new_code.push('\n');
                        }
                        self.code = new_code;
                        self.output = "Toggle Comment".to_string();
                        ui.close_menu();
                    }
                    if ui.button("⬅️ Decrease Indent").clicked() {
                        let mut new_code = String::new();
                        for line in self.code.lines() {
                            if line.starts_with("    ") {
                                new_code.push_str(&line[4..]);
                            } else {
                                new_code.push_str(line);
                            }
                            new_code.push('\n');
                        }
                        self.code = new_code;
                        self.output = "Decrease Indent".to_string();
                        ui.close_menu();
                    }
                    if ui.button("➡️ Increase Indent").clicked() {
                        let mut new_code = String::new();
                        for line in self.code.lines() {
                            new_code.push_str("    ");
                            new_code.push_str(line);
                            new_code.push('\n');
                        }
                        self.code = new_code;
                        self.output = "Increase Indent".to_string();
                        ui.close_menu();
                    }
                });
                // === VIEW MENU ===
                ui.menu_button("👁️ View", |ui| {
                    if ui.button("🔢 Toggle Line Numbers").clicked() {
                        self.output = "Line numbers not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🧹 Clear All").clicked() {
                        self.code.clear();
                        self.output.clear();
                        self.code_history = vec![String::new()];
                        self.code_history_index = 0;
                        ui.close_menu();
                    }
                    if ui.button("📝 Clear Code Editor").clicked() {
                        self.code.clear();
                        self.code_history = vec![String::new()];
                        self.code_history_index = 0;
                        ui.close_menu();
                    }
                    if ui.button("📊 Clear Output").clicked() {
                        self.output.clear();
                        ui.close_menu();
                    }
                    if ui.button("🐢 Clear Turtle Graphics").clicked() {
                        self.output = "Clear Turtle Graphics not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📝 Switch to Code Editor").clicked() {
                        self.tab = Tab::Editor;
                        ui.close_menu();
                    }
                    if ui.button("📊 Switch to Output").clicked() {
                        self.tab = Tab::Output;
                        ui.close_menu();
                    }
                    if ui.button("🐢 Switch to Turtle Graphics").clicked() {
                        self.output = "Switch to Turtle Graphics not implemented.".to_string();
                        ui.close_menu();
                    }
                });
                // === RUN MENU ===
                ui.menu_button("▶️ Run", |ui| {
                    if ui.button("🚀 Run Program").clicked() {
                        self.output = format!("[Output for {}]\n{}", self.language, self.code);
                        self.tab = Tab::Output;
                        ui.close_menu();
                    }
                    if ui.button("🛑 Stop Program").clicked() {
                        self.output = "Stop Program not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🧪 Run Tests").clicked() {
                        self.output = "Run Tests not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("📝 Check Syntax").clicked() {
                        self.output = "Check Syntax not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🔄 Restart Interpreter").clicked() {
                        self.output = "Restart Interpreter not implemented.".to_string();
                        ui.close_menu();
                    }
                });
                // === LANGUAGE MENU ===
                ui.menu_button("💻 Language", |ui| {
                    for lang in ["Time Warp", "Pascal", "Prolog", "Auto-Detect"] {
                        if ui.button(lang).clicked() {
                            self.language = lang.to_string();
                            self.output = format!("Language set to {}", lang);
                            ui.close_menu();
                        }
                    }
                });
                // === TOOLS MENU ===
                ui.menu_button("🛠️ Tools", |ui| {
                    if ui.button("🎨 Theme Selector...").clicked() {
                        self.output = "Theme Selector not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🔤 Font Settings...").clicked() {
                        self.output = "Font Settings not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📦 Compile to Executable...").clicked() {
                        self.output = "Compile to Executable not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("📦 Plugin Manager").clicked() {
                        self.output = "Plugin Manager not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("⚙️ Settings...").clicked() {
                        self.output = "Settings not implemented.".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📊 System Info").clicked() {
                        self.output = "System Info not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🧪 Test Suite").clicked() {
                        self.output = "Test Suite not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("📋 Generate Report").clicked() {
                        self.output = "Generate Report not implemented.".to_string();
                        ui.close_menu();
                    }
                });
                // === HELP MENU ===
                ui.menu_button("❓ Help", |ui| {
                    if ui.button("ℹ️ About Time_Warp IDE").clicked() {
                        self.output = "Time_Warp IDE v1.3.0\nEducational programming environment supporting multiple languages.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("📚 Documentation").clicked() {
                        self.output = "Documentation not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🌐 Online Resources").clicked() {
                        self.output = "Online Resources not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🆘 Report Issue").clicked() {
                        self.output = "Report Issue not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("💡 Feature Request").clicked() {
                        self.output = "Feature Request not implemented.".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🔄 Check for Updates").clicked() {
                        self.output = "Check for Updates not implemented.".to_string();
                        ui.close_menu();
                    }
                });
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Time Warp IDE");
                ui.separator();
                ui.label("Language:");
                for lang in ["BASIC", "PILOT", "Logo", "Pascal", "Prolog"] {
                    ui.selectable_value(&mut self.language, lang.to_string(), lang);
                }
                ui.separator();
                if ui.button("Editor").clicked() {
                    self.tab = Tab::Editor;
                }
                if ui.button("Output").clicked() {
                    self.tab = Tab::Output;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Run ▶").clicked() {
                        // Placeholder: In future, run interpreter here
                        self.output = format!("[Output for {}]\n{}", self.language, self.code);
                        self.tab = Tab::Output;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_height(500.0);
                match self.tab {
                    Tab::Editor => {
                        ui.heading("Code Editor");
                        ui.add_space(4.0);
                        ui.label(format!("Editing in {} mode", self.language));
                        ui.add_space(4.0);
                        ui.text_edit_multiline(&mut self.code);
                    }
                    Tab::Output => {
                        ui.heading("Output Console");
                        ui.add_space(4.0);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(&self.output);
                        });
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Time Warp IDE",
        options,
        Box::new(|_cc| Box::new(TimeWarpApp::default())),
    )
}

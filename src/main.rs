use eframe::{NativeOptions, egui, wgpu::naga::proc::IndexableLengthError};
enum Tombol {
    TombolLabel { time: u32 },
    NotPressed,
}
struct Crudapp {
    pressed_button: Tombol,
    input_task: String,
    list_task: Vec<String>,
}
impl eframe::App for Crudapp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Masukan Task");
                ui.text_edit_multiline(&mut self.input_task);
                ui.horizontal(|ui| {
                    if ui.button("Add task").clicked() {
                        self.list_task.push(self.input_task.clone());
                        self.input_task.clear();
                    }
                    if ui.button("Remove all Task").clicked() {
                        self.list_task.clear();
                    }
                });
            });
            let mut task_del: Option<usize> = None;
            ui.vertical(|ui| {
                ui.label("List Task");
                for (i, v) in self.list_task.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("No:{} [ {} ] ", i + 1, v));
                        if ui.button("remove").clicked() {
                            task_del = Some(i)
                        }
                    });
                }
            });
            if let Some(idx) = task_del {
                self.list_task.remove(idx);
            }
        });
    }
}
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    let app = Crudapp {
        pressed_button: Tombol::NotPressed,
        input_task: "".to_owned(),
        list_task: Vec::new(),
    };
    eframe::run_native("Hello", options, Box::new(|_cc| Ok(Box::new(app))))
}

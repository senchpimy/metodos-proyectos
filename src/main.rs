use eframe::egui;
use chrono;
use std::ffi::{c_char,CString};

#[repr(C)]
pub struct Raices {
    pub positivas:i32,
    pub negativas:i32,
}

extern "C"{
    fn numero_de_raices(test:*const c_char) -> Raices;
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Raiz de una Funcion",
        options,
        Box::new(|_cc| Box::<Proyecto1>::default()),
    )
}

struct Proyecto1 {
    funcion: String,
}

impl Default for Proyecto1 {
    fn default() -> Self {
        Self {
            funcion: "".to_owned(), // x^3-6x^2+11x^1-6
        }
    }
}

#[no_mangle]
pub extern fn create_string(val:&str) -> *const c_char {
    let c_string = CString::new(val).expect("CString::new failed");
    c_string.into_raw() // Move ownership to C
}

impl eframe::App for Proyecto1 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calcular Raiz de una funcion");
            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP),|ui|{
                ui.label(format!("{:?}", chrono::offset::Local::now()));
            } );
            ui.separator();
            ui.add(egui::TextEdit::singleline(&mut self.funcion));
            let raices = unsafe{numero_de_raices(create_string(&self.funcion))};
            ui.label(format!("Esta Funcion tiene {} raices positivas", raices.positivas));
            ui.label(format!("Esta Funcion tiene {} raices negativas", raices.negativas));
        });
        ctx.request_repaint();
    }
}

use eframe::egui;
use std::ffi::{c_char,CString};

#[repr(C)]
pub struct Raices {
    pub positivas:i32,
    pub negativas:i32,
}

extern "C"{
    fn numero_de_raices(str:*const c_char) -> Raices;
    fn str_to_int(str:*const c_char,index:i32) -> i32;
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
    funcion_compilada:Vec<(char, i32, i32)>
                    //operacion, multiplicacion, potencia
}

impl Default for Proyecto1 {
    fn default() -> Self {
        Self {
            funcion: "".to_owned(), // x^3-6x^2+11x^1-6
            funcion_compilada:Vec::new(),
        }
    }
}

#[no_mangle]
pub extern fn create_string(val:&str) -> *const c_char {
    let c_string = CString::new(val).expect("CString::new failed");
    c_string.into_raw() // Move ownership to C
}

fn compilar_funcion(funcion:&str, vec:&mut Vec<(char,i32,i32)>){
    let mut chars=funcion.chars();

    //let mut primera_operacion=('+',1,1);
    //let primer_elemento = chars.next().unwrap_or('+');
    //if primer_elemento=='+' {
    //    primera_operacion.0='+';
    //    primera_operacion.1=(chars.next().unwrap_or('0') as u32 - 48) as i32;
    //    let _ = chars.next();
    //    primera_operacion.2=(chars.next().unwrap_or('0') as u32 - 48) as i32;
    //}else if primer_elemento=='-' {
    //    primera_operacion.0='-';
    //    primera_operacion.1=(chars.next().unwrap_or('0') as u32 - 48) as i32;
    //    let _ = chars.next();
    //    primera_operacion.2=(chars.next().unwrap_or('0') as u32 - 48) as i32;
    //}
    //vec.push(primera_operacion);
    let mut total = 0;

    loop{
        let mut operacion=('+',1,1);
        let obtener = chars.next();
        let elemento = match obtener {
            Some(val)=>val,
            None=>{break}
        };
        if !ultimo_valor(funcion, total){
            if elemento=='+' {
                operacion.0='+';
                operacion.1=(chars.next().unwrap_or('0') as u32 - 48) as i32;
                let _ = chars.next();
                operacion.2=(chars.next().unwrap_or('0') as u32 - 48) as i32;
            }else if elemento=='-' {
                operacion.0='-';
                operacion.1=(chars.next().unwrap_or('0') as u32 - 48) as i32;
                let _ = chars.next();
                operacion.2=(chars.next().unwrap_or('0') as u32 - 48) as i32;
            }
            total+=2;
        }else{
            if elemento=='+' {
                operacion.0='+';
                operacion.1=(chars.next().unwrap_or('0') as u32 - 48) as i32;
                operacion.2=1;
            }else if elemento=='-' {
                operacion.0='-';
                operacion.1=(chars.next().unwrap_or('0') as u32 - 48) as i32;
                operacion.2=1;
            }

        }
        vec.push(operacion);
    }
}

fn ultimo_valor(funcion:&str,total:i32)->bool{
    let len = funcion.len();
    let (_, last) =funcion.split_at(len-total as usize);
    match last.find('^') {
        Some(_)=>return  true,
        None=>return false
    }
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
            if ui.add(egui::TextEdit::singleline(&mut self.funcion)).changed(){
                compilar_funcion(&self.funcion, &mut self.funcion_compilada);
                println!("{:?}",&self.funcion_compilada);
            }
            let raices = unsafe{numero_de_raices(create_string(&self.funcion))};
            ui.label(format!("Esta Funcion tiene {} raices positivas", raices.positivas));
            ui.label(format!("Esta Funcion tiene {} raices negativas", raices.negativas));
        });
        ctx.request_repaint();
    }
}

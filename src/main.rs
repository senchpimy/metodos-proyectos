use eframe::egui;
use std::ffi::{c_char,CString};
use std::str::Chars;
use egui::plot::{Line, Plot, PlotPoints};

#[repr(C)]
pub struct Raices {
    pub positivas:i32,
    pub negativas:i32,
}

extern "C"{
    fn numero_de_raices(str:*const c_char) -> Raices;
    //fn str_to_int(str:*const c_char,index:i32) -> i32;
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
    funcion_compilada:Vec<Ecuacion>,
    y:Vec<[f64;2]>,
                    //operacion, multiplicacion, potencia
}

impl Default for Proyecto1 {
    fn default() -> Self {
        Self {
            funcion: "+1x^3-6x^2+11x^1-6".to_owned(), // +1x^3-6x^2+11x^1-6
            funcion_compilada:Vec::new(),
            y:Vec::new(),
        }
    }
}

#[derive(Debug,Default)]
struct Ecuacion{
    positivo:i32,
    multiplicacion:i32,
    potencial:i32,
}

impl Ecuacion{
    fn evaluar(&self,x:f64)->f64{
        if self.positivo == 1 || self.positivo==-1{
            let pot = x.powi(self.potencial);
            return (pot*self.multiplicacion as f64)*self.positivo as f64;
        };
        self.positivo as f64
    }
}


#[no_mangle]
pub extern fn create_string(val:&str) -> *const c_char {
    let c_string = CString::new(val).expect("CString::new failed");
    c_string.into_raw() // Move ownership to C
}

fn consumir_chars(indice:&mut i32,mut numero:i32,chars:&mut Chars,extra:i32){
    loop{
        if numero<=9{
            *indice+=1;
            break
        }
        numero/=10;
        let _ = chars.next();
        *indice+=1;
        }
        for _ in 0..extra{
            let _ = chars.next();
            *indice+=1;
        }

}

fn crear_valores(val:&Vec<Ecuacion>, y:&mut Vec<[f64;2]>){
    for i in -50..80{
        let x = 0.1*i as f64;
        let mut coord:[f64;2]=Default::default();
        let mut y_actu = 0.0;
        for ec in val{
            y_actu+=ec.evaluar(x);
        }
        coord[0]=x;
        coord[1]=y_actu;
        y.push(coord);
    }

}

fn series(y:& Vec<[f64;2]>)->PlotPoints{
    PlotPoints::new(y.to_vec())
}

fn compilar_funcion(funcion:&str, vec:&mut Vec<Ecuacion>){
    let mut chars=funcion.chars();
    let mut indice = 1;

    loop{
        let mut operacion:Ecuacion=Default::default();
        //let str_ptr=create_string(funcion);
        let obtener = chars.next();
        let elemento = match obtener {
            Some(val)=>val,
            None=>{break}
        };
        if ultimo_valor(funcion, indice){ // +1x^3-6x^2+11x^1-6
            if elemento=='+' {
                operacion.positivo=1;
                let mul_val= str_to_int(funcion, indice);
                operacion.multiplicacion=mul_val;
                consumir_chars(&mut indice,mul_val,&mut chars,2);
                let pol_val= str_to_int(funcion, indice);
                operacion.potencial= pol_val;
                consumir_chars(&mut indice,pol_val,&mut chars,1);
            }else if elemento=='-' {
                operacion.positivo=-1;
                let mul_val= str_to_int(funcion, indice);
                operacion.multiplicacion=mul_val;
                consumir_chars(&mut indice,mul_val,&mut chars,2);
                let pol_val= str_to_int(funcion, indice);
                operacion.potencial= pol_val;
                consumir_chars(&mut indice,pol_val,&mut chars,1);
            }
            let _ = chars.next();
        }else{
            if elemento=='+' {
                operacion.positivo=1*str_to_int(funcion, indice);
                operacion.multiplicacion=0;
                operacion.potencial=0;
            }else if elemento=='-' {
                operacion.positivo=-1*str_to_int(funcion, indice);
                operacion.multiplicacion=0;
                operacion.potencial=0;
            }
            vec.push(operacion);
            break;
        }
        vec.push(operacion);
    }
}
fn str_to_int(s:&str, mut indice:i32)->i32{
    let mut ret=0;
    loop{
        let int = s.bytes().nth(indice as usize);
        let int = match int{
            Some(val)=>val,
            None=>{break}
        };
        if int>=57 || int<47 {break}
        ret=ret*10;
        ret+=(int-48) as i32;
        indice+=1;
    }
   return ret; 
}

fn ultimo_valor(funcion:&str,total:i32)->bool{
    let (_, last) =funcion.split_at((total-0) as usize);
    match last.find('^') {
        Some(_)=>return true,
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
                self.funcion_compilada=Vec::new();
                compilar_funcion(&self.funcion, &mut self.funcion_compilada);
                crear_valores(&self.funcion_compilada, &mut self.y);
            }
            let raices = unsafe{numero_de_raices(create_string(&self.funcion))};
            ui.label(format!("Esta Funcion tiene {} raices positivas", raices.positivas));
            ui.label(format!("Esta Funcion tiene {} raices negativas", raices.negativas));
            let line = Line::new(series(&self.y));
            Plot::new("my_plot").view_aspect(1.0).show(ui, |plot_ui| plot_ui.line(line));
        });
        ctx.request_repaint();
    }
}

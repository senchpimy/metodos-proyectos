use eframe::egui;
use egui::{TextFormat, Align};
use egui::text::LayoutJob;
use std::ffi::{c_char,CString};
use std::slice::Iter;
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
    funcion_compilada:Funcion,
    y:Vec<[f64;2]>,
                    //operacion, multiplicacion, potencia
    x_min:f64,
    x_max:f64,
    partes:i32,
    division_sintetica:DivisionSintetica,
    metodo_biseccion:MetodoBiseccion,
}

impl Default for Proyecto1 {
    fn default() -> Self {
        Self {
            funcion: "+1x^3-6x^2+11x^1-6".to_owned(), // +1x^3-6x^2+11x^1-6
            funcion_compilada:Funcion::default(),
            y:Vec::new(),
            x_min:-5.,
            x_max:5.,
            partes:100,
            division_sintetica:DivisionSintetica::default(),
            metodo_biseccion:MetodoBiseccion::default(),
        }
    }
}

#[derive(Debug,Default)]
struct Ecuacion{
    positivo:i32,
    multiplicacion:i32,
    potencial:i32,
}

#[derive(Debug,Default)]
struct Funcion{
    ecuaciones:Vec<Ecuacion>
}

impl Funcion {
    fn evaluar(&self, x:f64)->f64{
        let mut y =0.;
        for ec in &self.ecuaciones{
            y+=ec.evaluar(x);
        }
        y
    }
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

fn crear_valores(val:&Funcion, y:&mut Vec<[f64;2]>,mut min:f64, max:f64, partes:i32){
    let diff = (min - max).powi(2).sqrt();
    let paso = diff /partes as f64;
    for _ in 0..partes{
        min += paso;
        let mut coord:[f64;2]=Default::default();
        let y_actu = val.evaluar(min);
        coord[0]=min;
        coord[1]=y_actu;
        y.push(coord);
    }

}

fn series(y:& Vec<[f64;2]>)->PlotPoints{
    PlotPoints::new(y.to_vec())
}

fn compilar_funcion(funcion:&str, vec:&mut Funcion){
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
                operacion.positivo=str_to_int(funcion, indice);
                operacion.multiplicacion=0;
                operacion.potencial=0;
            }else if elemento=='-' {
                operacion.positivo=-str_to_int(funcion, indice);
                operacion.multiplicacion=0;
                operacion.potencial=0;
            }
            vec.ecuaciones.push(operacion);
            break;
        }
        vec.ecuaciones.push(operacion);
    }
}
fn str_to_int(s:&str, mut indice:i32)->i32{
    let mut ret=0;
    loop{
        let int = s.as_bytes().get(indice as usize).copied();
        let int = match int{
            Some(val)=>val,
            None=>{break}
        };
        if !(47..57).contains(&int) {break}
        ret *= 10;
        ret+=(int-48) as i32;
        indice+=1;
    }
   ret
}

fn recursive_append(s:&mut Chars,upper:&mut bool,job:&mut LayoutJob){
    let element = s.next();
    let char = match element {
        Some(val)=>val,
        None=>{return;}
    };
    if *upper && (char == '+' || char == '-'){
        *upper=false;
    }
    if !*upper{
        if char == '^'{
            *upper=true;
        }else{
            job.append(&format!("{}",char), 0.0, TextFormat::default());
        }
    }else{
        job.append(&format!("{}",char), 0.0, TextFormat { 
            font_id: egui::FontId { size: 10., ..Default::default() },
            valign:Align::TOP,
            ..Default::default()
        });
    }
    recursive_append(s, upper, job);
}

fn func_to_gui(s:&str)->LayoutJob{
    let mut job = LayoutJob::default();
    job.append("Y= ", 0.0, TextFormat::default());
    match s.find('^') {
        Some(_)=>{
            let mut bo=false;
            let mut ss = s.chars();
            recursive_append(&mut ss,&mut bo,&mut job)
        },
        None=>{
            job.append(s, 0.0, TextFormat::default())
        }
        
    } 
    job
}

fn ultimo_valor(funcion:&str,total:i32)->bool{
    let (_, last) =funcion.split_at(total as usize);
    match last.find('^') {
        Some(_)=>true,
        None=>false
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
                self.funcion_compilada=Funcion::default();
                self.y=Vec::new();
                compilar_funcion(&self.funcion, &mut self.funcion_compilada);
                crear_valores(&self.funcion_compilada, &mut self.y, self.x_min, self.x_max, self.partes);
                self.division_sintetica.actualizar_datos(&self.funcion_compilada);
                self.division_sintetica.obtener_resultados();
            }
            let raices = unsafe{numero_de_raices(create_string(&self.funcion))};
            ui.separator();
            ui.label(func_to_gui(&self.funcion));
            ui.separator();
            ui.label(format!("Esta Funcion tiene {} raices positivas", raices.positivas));
            ui.label(format!("Esta Funcion tiene {} raices negativas", raices.negativas));
            if ui.add(egui::Slider::new(&mut self.x_min, -70.0..=self.x_max-2.).text("Valor Minimo de X")).changed() ||
            ui.add(egui::Slider::new(&mut self.x_max, (self.x_min+2.)..=70.0).text("Valor Maximo de X")).changed() ||
            ui.add(egui::Slider::new(&mut self.partes, 1..=1500).text("Numero de Partes")).changed() {
                self.y=Vec::new();
                crear_valores(&self.funcion_compilada, &mut self.y, self.x_min, self.x_max, self.partes);
            }
            ui.separator();
                ui.label("Metodo de division Sintetica");
                for res in &self.division_sintetica.resultados{
                    match  res.1 {
                        Some(val) =>
                            if val !=0{
                            ui.label(format!("El valor {} pudo haber sido una raiz pero si valor en Y es de {}",res.0,val));

                            }else{
                                ui.label(format!("X: {} Y: {}",res.0, val));
                            }
                        None => {}
                    };
                }
            ui.label(format!("Terminos Independientes: {:?}",&self.division_sintetica.terminos_in));
            ui.label(format!("Terminos Factores: {:?}",&self.division_sintetica.factores));
            ui.separator();
                ui.label("Metodo del Conejo (Metodo de Biseccion)");
            if ui.button("Activa").clicked(){
                self.metodo_biseccion.buscar_raiz(&self.funcion_compilada,3);
                self.metodo_biseccion.calcular_raices(&self.funcion_compilada);
            }
            ui.separator();
            let line = Line::new(series(&self.y)).width(5.);
            Plot::new("Plot").view_aspect(0.1).show(ui, |plot_ui| plot_ui.line(line));
        });
        ctx.request_repaint();
    }
}
#[derive(Default, Debug)]
struct DivisionSintetica{
    factores:Vec<i32>,
    terminos_in:Vec<i32>, 
    resultados:Vec<(i32,Option<i32>)>, // No se toca
}

impl DivisionSintetica{
    fn obtener_resultados(&mut self){
        self.resultados=Vec::new();
        for factor in &self.factores{
            self.resultados.push(division_sin(factor, &self.terminos_in))
        }
    }
    fn actualizar_datos(&mut self, datos:&Funcion){
        let term_in= match datos.ecuaciones.last(){
            Some(a)=>a.positivo.abs(),
            None=>return
        };
        let term_in_factors=get_factors(term_in);
        let mut max = 0;
        for ec in &datos.ecuaciones{
            if ec.potencial>max{
                max = ec.potencial;
            }
        }
        let mut max_factors=get_factors(max);
        max_factors.pop();
        self.factores=Vec::new();
        for fac in term_in_factors{
            for max_fac in &max_factors{
                let val = fac%max_fac;
                if val == 0{self.factores.push(fac/max_fac)}
            }
        }
        let h:Vec<i32> = datos.ecuaciones.iter().map(|dat|{
            if dat.positivo.abs()>1{
                return dat.positivo
            }
            return dat.positivo*dat.multiplicacion
        }).collect();
        self.terminos_in=h;
    }
}

fn get_factors(n: i32) -> Vec<i32> {
    (1..n+1 ).into_iter().filter(|&x| n % x == 0).collect::<Vec<i32>>()
}

fn division_sin(factor:&i32, term_in:&Vec<i32>)->(i32,Option<i32>){
    let mut terminos_in_iter = term_in.iter();
    let prim_term_in = match terminos_in_iter.next(){
        Some(val)=>val,
        None=>return (*factor, None)
        };
    let res = resta(&mut terminos_in_iter, factor,prim_term_in);
    (*factor,Some(res))
}

fn resta(iters:&mut Iter<i32>, factor:&i32, num:&i32)->i32{
    let mult = factor*num;
    let int = match iters.next(){
        Some(val)=>val,
        None=>return *num
    };
    let res = int+mult;
    resta(iters, factor, &res)
}

#[derive(Debug,Default)]
struct MetodoBiseccion{
    raices:Vec<Biseccion>,
    resultados:Vec<[f64;2]>
}

impl MetodoBiseccion{
    fn buscar_raiz(&mut self,funcion:&Funcion, num_raices:i32){// Busca dos numeros de a y b tal que f(a)*f(b)<0
        let mut min=-5.;
        let paso = 0.33454;
        let mut bisecciones:Vec<Biseccion>=Vec::new();
        for _ in 0..num_raices{
            let mut max = min+paso;
            loop{
                let fa = funcion.evaluar(min);
                let fb = funcion.evaluar(max);
                if fa*fb<0.{
                    let bi= Biseccion::new(min, max+0.247958);
                    bisecciones.push(bi);
                    min=max+0.247958;
                    break
                }
                max += paso;
            }
        }
        self.raices=bisecciones;
    }

    fn calcular_raices(&mut self,funcion:&Funcion){
        let tolerancia = 0.15;
        let tolerancia2=0.05;
        self.resultados=Vec::new();
        for bi in &self.raices{
            let (mut b, mut a) = bi.desarmar();
            for _ in 0..150 {
                let x0 = (a+b)/2.;
                let y = funcion.evaluar(x0);
                if ((0.-tolerancia)..(0.+tolerancia)).contains(&y){
                    self.resultados.push([x0,y]);
                    println!("EContrado 1 {:?}", &self.resultados);
                    break;
                }
                let fa = funcion.evaluar(a);
                let fb = funcion.evaluar(b);
                let fx=funcion.evaluar(x0);
                if x0>0.{
                    if (fa*fx)>0.{
                        b += tolerancia2;
                        continue;
                    }
                    b = x0;
                }else {
                    if (fb*fx)>0.{
                        a += tolerancia2;
                        continue;
                    }
                    a = x0;
                }

            }
        }
    }
}

#[derive(Debug)]
struct Biseccion{
    max:f64,
    min:f64
}

impl Biseccion{
    fn new(min:f64, max:f64)->Biseccion{
        Biseccion { max, min }
    }
    fn desarmar(&self)->(f64,f64){
        return (self.max, self.min);
    }
}

use eframe::egui;
use egui::{TextFormat, Align};
use egui_extras::{Column, TableBuilder};
use egui::text::LayoutJob;
use std::ffi::{c_char,CString};
use std::slice::Iter;
use std::str::Chars;
use egui::plot::{Line, Plot, PlotPoints};
//-------------------------------------- Incorporar el codigo de C
#[repr(C)]
pub struct Raices {
    pub positivas:i32,
    pub negativas:i32,
}

#[no_mangle]
pub extern fn create_string(val:Option<&str>) -> *const c_char {
    match val {
        Some(val)=>{
            let c_string = CString::new(val).expect("CString::new failed");
            c_string.into_raw() // Move ownership to C
        },
        None=>{
            CString::new("No value").expect("CString::new failed").into_raw()
        }
    }
}

extern "C"{
    fn numero_de_raices(str:*const c_char) -> Raices;
}
//-------------------------------------- Incorporar el codigo de C

//-------------------------------------- Iniciar el programa
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

//-------------------------------------- Estructura para la interfaz grafica
struct Proyecto1 {
    funcion: String, // Donde se guarda la funcion
    funcion_compilada:Funcion, // La funcion para evaluar
    y:Vec<[f64;2]>, //Coordenadas para la grfica
    x_min:f64, //Valor minimo para la grafica
    x_max:f64, //Valor maximo para la grafica
    partes:i32, // Cuantas partes tiene la grafica
    division_sintetica:DivisionSintetica, // Division Sintetica
    metodo_biseccion:MetodoBiseccion, // Metodo de biseccion
    metodo_newton:Newton,
}

//-------------------------------------- Estructura para la interfaz grafica
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
            metodo_newton:Newton::default(),
        }
    }
}

//-------------------------------------- Estructura para grafica
#[derive(Debug,Default,Clone)]
struct Ecuacion{
    positivo:i32,
    multiplicacion:i32,
    potencial:i32,
}

//-------------------------------------- Estructura para grafica
#[derive(Debug,Default,Clone)]
struct Funcion{
    ecuaciones:Vec<Ecuacion>
}

//-------------------------------------- Estructura para grafica
impl Funcion {
    fn evaluar(&self, x:f64)->f64{
        let mut y =0.;
        for ec in &self.ecuaciones{
            y+=ec.evaluar(x);
        }
        y
    }
    fn to_derivada(&self)->Funcion{
        let mut fun = self.clone();
        for eq in &mut fun.ecuaciones{
            if eq.positivo.abs()>1{
                continue;
            }
            let mul = eq.potencial;
            eq.multiplicacion*=mul;
            eq.potencial-=1;
        }
        fun.ecuaciones.pop();
        let len = fun.ecuaciones.len();
        fun.ecuaciones[len-1].positivo = fun.ecuaciones[len-1].multiplicacion*fun.ecuaciones[len-1].positivo ;
        fun.ecuaciones[len-1].multiplicacion=0;
        fun
    }
}

//-------------------------------------- Estructura para grafica
impl Ecuacion{
    fn evaluar(&self,x:f64)->f64{
        if self.positivo == 1 || self.positivo==-1{
            let pot = x.powi(self.potencial);
            return (pot*self.multiplicacion as f64)*self.positivo as f64;
        };
        self.positivo as f64
    }
}



//-------------------------------------- Funciones para la grafica (Termina en la linea 258)
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
//-------------------------------------- Funciones para la grafica

//-------------------------------------- Funcion de la Interfaz grafica
impl eframe::App for Proyecto1 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calcular Raiz de una funcion"); // Titulo
            ui.separator(); // Separardor
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP),|ui|{ // Reloj
                ui.label(format!("{:?}", chrono::offset::Local::now()));// Reloj
            } );// Reloj
            ui.separator();// Separardor
            if ui.add(egui::TextEdit::singleline(&mut self.funcion)).changed(){ // Espacio para
                // ingresar la funcion
                // Cada que se agregue o elimine algo de este espacio, lo siguiente se ejecuta
                self.funcion_compilada=Funcion::default(); // Se actualiza la grafica
                self.y=Vec::new();// Se actualiza la grafica
                compilar_funcion(&self.funcion, &mut self.funcion_compilada);// Se actualiza la grafica
                crear_valores(&self.funcion_compilada, &mut self.y, self.x_min, self.x_max, self.partes);// Se actualiza la grafica
                self.division_sintetica.actualizar_datos(&self.funcion_compilada); // Se actualizan
                // los datos de la funcion compilada
                self.division_sintetica.obtener_resultados(); // Se obtienen los resultados
                self.metodo_newton.obtener_derivada(self.funcion_compilada.clone());
                self.metodo_newton.obtener_raices();
            }
            // Se prepara el String de la fucnion para enviarse a C
            let c_string= if self.funcion.len()>0{
                let first = self.funcion.clone().remove(0);
                if first == '+' {
                    let (_,c) = self.funcion.split_at(1);
                    Some(c)
                }else{
                    Some(self.funcion.as_str())
                }
            }else{
                None
            };
            let raices = unsafe{numero_de_raices(create_string(c_string))}; // Se obtienen los resultados de contar cuantas raices positivas o negativas
            ui.separator(); //Separardor
//            ui.label(func_to_gui(&self.funcion)); // Se muestra la funcion de forma estilizada
//            ui.separator(); //Separardor
//            // Se dice cuantas raices tuvo la funcion
//            ui.label(format!("Esta Funcion tiene {} raices positivas", raices.positivas));
//            ui.label(format!("Esta Funcion tiene {} raices negativas", raices.negativas));
//            let num_raices=raices.positivas+raices.negativas; // Toal de posibles raices
//
//            // Modificadores para la grafica
//            if ui.add(egui::Slider::new(&mut self.x_min, -70.0..=self.x_max-2.).text("Valor Minimo de X")).changed() ||
//            ui.add(egui::Slider::new(&mut self.x_max, (self.x_min+2.)..=70.0).text("Valor Maximo de X")).changed() ||
//            ui.add(egui::Slider::new(&mut self.partes, 1..=1500).text("Numero de Partes")).changed() {
//                self.y=Vec::new();
//                crear_valores(&self.funcion_compilada, &mut self.y, self.x_min, self.x_max, self.partes);
//            }
//
            ui.separator(); //Separardor
//                ui.label("Metodo de division Sintetica"); // Titulo de la funcion
//                // Por todos los resultados de la  division_sintetica
//                for res in &self.division_sintetica.resultados{ 
//                    match  res.1 { // Si el valor en Y  es valido
//                        Some(val) => 
//                            if val !=0{ // Si es diferente de 0 no es raiz 
//                            ui.label(format!("El valor {} pudo haber sido una raiz pero si valor en Y es de {}",res.0,val));
//                            }else{ // Si es igual a 0 entonces si es una raiz
//                                ui.label(format!("X: {} Y: {}",res.0, val));
//                            }
//                        None => {}
//                    };
//                }
//
//            ui.label(format!("Terminos Independientes: {:?}",&self.division_sintetica.terminos_in)); // Mostramos los terminos Independientes
//            ui.label(format!("Terminos Factores: {:?}",&self.division_sintetica.factores));  // Mostramos los Factores
//            ui.separator(); // Separador
//                ui.label("Metodo del Conejo (Metodo de Biseccion)"); // Metodo de la biseccion
//                if ui.button("Usar").clicked() && self.funcion_compilada.ecuaciones.len()>0{ 
//                     // Cuando
//                     // se haga click en el boton y la ecuacion que ingr4eso el usuario sea valida se
//                     // ejecutara lo siguiente
//                     
//                     // Se buscan
//                     // las raicez
//                     self.metodo_biseccion.buscar_raiz(&self.funcion_compilada,num_raices); 
//
//                     // Se calculan las raices
//                     self.metodo_biseccion.calcular_raices(&self.funcion_compilada);
//                }
//                for res in &self.metodo_biseccion.resultados{ // Se muestran los resultados
//                    ui.label(format!("Raiz encontrada en: {}
//    Con un valor en Y de: {}",res[0], res[1])) ;
//                }
            ui.separator();
            ui.label("Metodo Newton-Raphson");
            let table = TableBuilder::new(ui)
                            .striped(true)
                            .column(Column::auto())
                            .column(Column::initial(200.0))
                            .column(Column::initial(200.0))
                            .column(Column::initial(200.0))
            ;
            table.header(20.0, |mut header|{
                header.col(|ui|{ui.strong("Iteracion");});
                header.col(|ui|{ui.strong("X");});
                header.col(|ui|{ui.strong("F(x)");});
                header.col(|ui|{ui.strong("F'(x)");});
            }).body(|mut body|{
                for val in &self.metodo_newton.resultados{
                     body.row(20.0, |mut row|{
                        row.col(|ui|{ui.label(val.3.to_string());});
                        row.col(|ui|{ui.label(val.2.to_string());});
                        row.col(|ui|{ui.label(val.0.to_string());});
                        row.col(|ui|{ui.label(val.1.to_string());});
                         })  
                }
            });
            
            ui.separator();
            let line = Line::new(series(&self.y)).width(5.); // Se hacen las lineas del grafico 
            Plot::new("Plot").view_aspect(0.1).show(ui, |plot_ui| plot_ui.line(line)); // Se
            // muestra el grafico
        });
        ctx.request_repaint(); // Se repite el proceso
    }
}


//-------------------------------------- Inicia el metodo de DivisionSintetica
#[derive(Default, Debug)]
struct DivisionSintetica{
    factores:Vec<i32>, // Factores de los numeros
    terminos_in:Vec<i32>,  // Terminos Independientes
    resultados:Vec<(i32,Option<i32>)>, // No se toca
}

// Metodos
impl DivisionSintetica{
    fn obtener_resultados(&mut self){
        self.resultados=Vec::new(); // Eliminamos los resultados anteriores
        for factor in &self.factores{ // Por cada factor
            self.resultados.push(division_sin(factor, &self.terminos_in)) // Obtenemos el
            // resultado de ese factor con los terminos Independientes
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

// Funcion que obtiene los factores de un numero
fn get_factors(n: i32) -> Vec<i32> {
    (1..n+1 ).into_iter().filter(|&x| n % x == 0).collect::<Vec<i32>>() // En un rango de 1 hasta
    // n+1 seleccioneamos los valores que divididos por n den 0 ( sean factores)
}

// Division Sintetica
fn division_sin(factor:&i32, term_in:&Vec<i32>)->(i32,Option<i32>){
    let mut terminos_in_iter = term_in.iter();
    let prim_term_in = match terminos_in_iter.next(){
        Some(val)=>val,
        None=>return (*factor, None)
        };
    let res = resta(&mut terminos_in_iter, factor,prim_term_in);
    (*factor,Some(res))
}

// Suma la multiplicacion del factor por el temino independiente
fn resta(independientes:&mut Iter<i32>, factor:&i32, num:&i32)->i32{
    let mult = factor*num;
    let int = match independientes.next(){
        Some(val)=>val,
        None=>return *num
    };
    let res = int+mult;
    resta(independientes, factor, &res)
}

//-------------------------------------- Inicia el metodo de Biseccion
#[derive(Debug,Default)]
struct MetodoBiseccion{
    raices:Vec<Biseccion>,
    resultados:Vec<[f64;2]>
}

impl MetodoBiseccion{
    // Obtenemos los segmentos en los cuales se va a buscar la raiz
    fn buscar_raiz(&mut self,funcion:&Funcion, num_raices:i32){// Busca dos numeros de a y b tal que f(a)*f(b)<0
        let mut min=-5.;
        let paso = 0.33454;
        let max_iter=1000;
        let mut bisecciones:Vec<Biseccion>=Vec::new();
        for _ in 0..num_raices{
            let mut max = min+paso;
            for _ in 0..max_iter{
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

    // Obtenemos la raiz
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

//-------------------------------------- Inicia el metodo de Newton-Raphson

#[derive(Default)]
struct Newton{
    derivada:Funcion,
    funcion:Funcion,
    resultados:Vec<(f64,f64,f64,i32)>
}

impl Newton {
   fn obtener_derivada(&mut self, fun:Funcion){
        self.funcion=fun.clone();
        self.derivada=fun.to_derivada();
    }

    fn obtener_raices(&mut self){
        self.resultados=Vec::new();
        let mut x0 = 0.0;
        let tolerancia = 0.0001;
        for i in 0..50{
            let fx=self.funcion.evaluar(x0);
            let fdx=self.derivada.evaluar(x0);
            let div = fx/fdx;
            println!("x={}; fx={}; iter= {}",x0,fx,i);
            self.resultados.push((fx,fdx,x0,i));
            x0 = x0-div;
            if fx.abs()<tolerancia{break;}
        }
    }
}

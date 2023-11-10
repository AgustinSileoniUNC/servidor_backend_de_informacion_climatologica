
use std::time::Instant;

use crate::{regular_expressions::{obtain_report_tiempo_presente, obtain_estacion_pronostico, identify_estacion_line, report_dato_horario, report_pronostico, identify_data_line_pronostico, identify_data_line_dato_horario}, models::{TiempoPresente, Pronostico, DatoHorario}};




pub fn filter_data_tiempo_presente(data:String)-> Vec<TiempoPresente>{

    let mut reports_tiempo_presente: Vec<TiempoPresente> = Vec::new();
    
    let lines = data.lines();

    for (_ , line) in lines.enumerate() {
        reports_tiempo_presente.push( obtain_report_tiempo_presente(evite_empty_values(line.to_string())) );
    }

    return reports_tiempo_presente;
}


fn evite_empty_values(original_text_line:String)-> String{

    let mut text_line= original_text_line.to_string();

    if original_text_line.contains(";;"){
        text_line = original_text_line.replace(";;", ";0 km;");
    }

    return text_line.to_string();

}


pub fn filter_data_pronostico(data:String)-> Vec<Pronostico>{
    
    let mut reports_pronostico: Vec<Pronostico> = Vec::new();
    let mut lines = data.lines();
    
    //Delete header
    for _  in 1..6{
        lines.next();
    }

    let mut name_estacion = String::new();

    for (_, line) in lines.enumerate(){
        if identify_estacion_line(line.to_owned()){
            name_estacion= obtain_estacion_pronostico(line.to_string());
        }
        else if identify_data_line_pronostico(line.to_owned()){
            let report = report_pronostico(name_estacion.to_owned(), line.to_owned());
            reports_pronostico.push(report);
        }
    }

    return reports_pronostico;


}


pub fn filter_data_dato_horario(data:String)-> Vec<DatoHorario>{
    
    let mut reports_dato_horario : Vec<DatoHorario> = Vec::new();
    let mut lines = data.lines();

    //Delete header
     for _ in 1..3{
         lines.next();
     }

     let start_time = Instant::now();
 
    for (_, line) in lines.enumerate(){

        if identify_data_line_dato_horario(line.to_owned()){
            reports_dato_horario.push(report_dato_horario(line.to_owned()));
        }
    }

    let end_time_data = Instant::now();
    let duration = end_time_data.duration_since(start_time);
    println!("El procesamiento de lineas tomo {} segundos", duration.as_secs());


    return reports_dato_horario;
}
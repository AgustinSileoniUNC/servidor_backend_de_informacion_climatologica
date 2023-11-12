
use std::collections::HashMap;

use crate::{regular_expressions::{obtain_report_tiempo_presente, obtain_estacion_pronostico, identify_estacion_line, report_dato_horario, report_pronostico, identify_data_line_pronostico, identify_data_line_dato_horario}, models::{TiempoPresente, Pronostico, DatoHorario}};




pub fn filter_data_tiempo_presente(data:String)-> HashMap<String,Vec<TiempoPresente>>{

    let mut tiempo_presente :HashMap<String, Vec<TiempoPresente>> = HashMap::new();
    let mut reports_tiempo_presente: Vec<TiempoPresente> = Vec::new();
    
    let mut lines = data.lines();

    let  first_report:TiempoPresente =obtain_report_tiempo_presente(evite_empty_values(lines.next().unwrap().to_string()));
    reports_tiempo_presente.push( first_report);
    
    
    for (_ , line) in lines.enumerate() {

        let report = obtain_report_tiempo_presente(evite_empty_values(line.to_string())) ;

        if report.estacion.eq(&reports_tiempo_presente.last().unwrap().estacion){
            reports_tiempo_presente.push( report);
        }
        else{
            tiempo_presente.insert(reports_tiempo_presente.last().unwrap().estacion.to_string(), reports_tiempo_presente.clone());
            reports_tiempo_presente.clear();
            reports_tiempo_presente.push(report);
        }

    }

    tiempo_presente.insert(reports_tiempo_presente.last().unwrap().estacion.to_string(), reports_tiempo_presente.clone());
    

    return tiempo_presente;
}


fn evite_empty_values(original_text_line:String)-> String{

    let mut text_line= original_text_line.to_string();

    if original_text_line.contains(";;"){
        text_line = original_text_line.replace(";;", ";0 km;");
    }

    return text_line.to_string();

}


pub fn filter_data_pronostico(data:String)-> HashMap< String,Vec<Pronostico>>{

    let mut pronostico : HashMap< String,Vec<Pronostico> > = HashMap::new();
    let mut reports_pronostico: Vec<Pronostico> = Vec::new();
    let mut lines = data.lines();

    //Delete header
    for _  in 1..6{
        lines.next();
    }

    let mut name_estacion = obtain_estacion_pronostico(lines.next().unwrap().to_string());

    for ( _, line) in lines.enumerate(){
        
        let line_filtered = filter_no_ascii(line);

        if identify_estacion_line(line_filtered.to_owned()){
            pronostico.insert(name_estacion.to_string(), reports_pronostico.clone());
            reports_pronostico.clear();
            name_estacion= obtain_estacion_pronostico(line_filtered.to_string()); 
        }
        else if identify_data_line_pronostico(line_filtered.to_owned()){
            let report = report_pronostico(name_estacion.to_owned(), line_filtered.to_owned());
            reports_pronostico.push(report);
        }
    }

    pronostico.insert(name_estacion.to_string(), reports_pronostico.clone());

    return pronostico;

}

fn filter_no_ascii(text_line:&str)-> String{
    let first_filter = text_line.replace("Ã‘","N");
    let output = first_filter.replace(".", "_");
    return output;
}


pub fn filter_data_dato_horario(data:String)->Vec<DatoHorario>{

    let mut reports_dato_horario : Vec<DatoHorario> = Vec::new();
    let mut lines = data.lines();
    //Delete header
     for _ in 1..3{
         lines.next();
     }


 
    for (_, line) in lines.enumerate(){

        if identify_data_line_dato_horario(line.to_owned()){

            reports_dato_horario.push(report_dato_horario(line.to_owned()));
        }
    }

    return reports_dato_horario;
}
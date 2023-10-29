
use crate::{regular_expressions::{obtain_report_tiempo_presente, obtain_estacion_pronostico, obtain_struct_pronostico, identify_division_line, identify_data_line, identify_estacion_line}, structs::{TiempoPresente, Pronostico}};




pub fn filter_data_tiempo_presente(data:String)-> Vec<TiempoPresente>{

    let mut reports_tiempo_presente: Vec<TiempoPresente> = Vec::new();
    
    let lines = data.lines();

    for (_ , line) in lines.enumerate() {
        let valor = obtain_report_tiempo_presente(evite_empty_values(line.to_string()));
        //println!("{} - {:?}", number, valor);
        reports_tiempo_presente.push( valor );
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

    //let mut reports_pronostico: Vec<Pronostico> = Vec::new();
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
        else if identify_data_line(line.to_owned()){
            let report = obtain_struct_pronostico(name_estacion.to_owned(), line.to_owned());
            println!("{:?}", report);
            reports_pronostico.push(report);
        }
    }

    return reports_pronostico;


}
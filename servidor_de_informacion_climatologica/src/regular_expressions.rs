
use regex::Regex;

use crate::models::{TiempoPresente, Pronostico, DatoHorario};


pub fn obtain_report_tiempo_presente(texto:String)-> TiempoPresente{
    
    let regular_expression =  Regex::new(r"(\w+(\D*)*(\w+)*)(;)(\d{2}-\w+-\d{4})(;)(\d{2}:\d{2})(;)(\w+(\s\w+)*)(;)(\d+(\.\d+)*)(\s(km))(;)((-)*\d+((\.\d+)*))(;)((No se calcula)|((-)*\d+\.*\d*))(;\s)(\d+)(;)(\w+(\s\w+)*(\s\s\w+)*)(;)(\d+(\.\d+)*)").unwrap();


    match regular_expression.captures(&texto){
        Some(caps) => {
            TiempoPresente{
                estacion: caps[1].to_owned(),
                fecha: caps[5].to_owned(),
                hora: caps[7].to_owned(),
                estado_nuboso: caps[9].to_owned(),
                viento_intensidad:  caps[12].to_owned(),
                temperatura: caps[17].to_owned(),
                sensacion_termica: caps[22].to_owned(),
                humedad_relativa:caps[27].to_owned(),
                viento_direccion: caps[29].to_owned(),
                presion_superficie: caps[33].to_owned(),
            }
        },
        None => { 
                None.unwrap()
        }    
    }
}



pub fn report_pronostico( estacion:String, texto:String) -> Pronostico{

     //let nombre_estacion = Regex::new(r"([\w*().]+)$").unwrap();
     let regular_expression = Regex::new(r"(\d{2})(\d{2})(\d{4})(\s*)(\d+)(\s*)((-)*\d*\.\d+)(\s*)(\d*)*(\s*)(((\d+\.\d)*))(\s*)(\d*)(\s*)(\d*)(\s*)(\w+(\D*)*(\w+)*)").unwrap();
     //Elimina la Ñ porque no la registra bien
     //Ã‘ es la forma en la que lee la ñ
     //modificada = linea.replace("Ã‘","N");

     match regular_expression.captures(&texto){
        Some(caps) => {
            Pronostico{
                estacion,
                fecha: caps[1].trim().to_owned(),
                hora: caps[2].trim().to_owned(),
                temperatura: caps[4].trim().to_owned(),
                viento_direccion: caps[5].trim().to_owned(),
                viento_intensidad:  caps[7].trim().to_owned(),
                precipitacion: caps[8].trim().to_owned()
            }
        },
        None => { 
                None.unwrap()
        }    
    }
}



pub fn report_dato_horario(texto:String)-> DatoHorario{

    let regular_expression = Regex::new(r"(\d{8})(\s*)(\d+)(\s*)((-)*\d*\.\d+)(\s*)(\d*)*(\s*)(((\d+\.\d)*))(\s*)(\d*)(\s*)(\d*)(\s*)(\w+(\D*)*(\w+)*)").unwrap();
    match regular_expression.captures(&texto) {
        Some(caps)=>{

            DatoHorario{
                estacion: caps[18].trim().to_owned(),
                fecha: caps[1].to_owned(),
                hora: caps[3].to_owned(),
                temperatura: caps[5].to_owned(),
                humedad_relativa: caps[8].to_owned(),
                presion_superficie: caps[10].to_owned(),
                viento_direccion: caps[14].to_owned(),
                viento_intensidad: caps[16].to_owned()
            }
        }
        None => {
            None.unwrap()
        }
    }    
}


pub fn obtain_estacion_pronostico(texto:String)-> String{
    
    let nombre_estacion: Regex = Regex::new(r"([\w*().]+)$").unwrap();

    match nombre_estacion.captures(&texto){
        Some(caps) => {
            return caps[0].to_string();
        },
        None => { 
             return "".to_string();   
        }    
    }
}

pub fn identify_data_line_pronostico(line:String)->bool{
    let regular_expression = Regex::new(r"(\d{2}/\w{3}/\d{4}\s*)(\d{2})(Hs.\s*)(\d+.\d\s*)(\d+)(\s[|]\s*)(\d+\s*)(\d+.\d)").unwrap();

    return regular_expression.is_match(&line);

}

pub fn identify_data_line_dato_horario(line:String)-> bool{
    let regular_expression = Regex::new(r"(\d{8})(\s*)(\d+)(\s*)((-)*\d*\.\d+)(\s*)(\d*)*(\s*)(((\d+\.\d)*))(\s*)(\d*)(\s*)(\d*)(\s*)(\w+(\D*)*(\w+)*)").unwrap();

    return regular_expression.is_match(&line);

}
pub fn identify_estacion_line(line:String)-> bool{
    let regular_expression_estacion_line = Regex::new(r"([\w*_*]+)$").unwrap();

    if regular_expression_estacion_line.is_match(&line){
        return true;
    }
    else{
        return false;
    }
}

pub fn identify_division_line(texto:String)-> bool{
    
    let regular_expression = Regex::new(r"(={96})").unwrap();

    match regular_expression.captures(&texto){
        Some(_) => {
            return true;
        },
        None => { 
             return false;
        }    
    }
}
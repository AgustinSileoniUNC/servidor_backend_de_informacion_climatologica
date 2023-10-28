
use regex::Regex;

use crate::structs::TiempoPresente;





pub fn obtener_estructura_tiempo_presente(texto:String)-> TiempoPresente{
    
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




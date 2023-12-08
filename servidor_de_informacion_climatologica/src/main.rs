use std::collections::HashMap;

use servidor_de_informacion_climatologica::{update_data::{update_datos_tiempo_presente, update_datos_pronostico}, handler_estaciones::{obtain_nombre_estacion_tiempo_presente, obtain_nombre_estacion_pronostico}, models::{TiempoPresente, Pronostico}};



fn main() {


    
    let data_tiempo_presente:HashMap<String, Vec<TiempoPresente>>;
    let  datos_pronostico: HashMap<String,Vec<Pronostico>>;

    data_tiempo_presente =  update_datos_tiempo_presente();
    datos_pronostico = update_datos_pronostico();


   let result = consult_tiempo_presente("concordia", data_tiempo_presente);

    println!("{}",result);

    let pronostico = consult_pronostico("concordia", datos_pronostico);
    
    println!("{}", pronostico);

}







fn consult_tiempo_presente(alias_estacion:&str, data_tiempo_presente:HashMap<String,Vec<TiempoPresente>>)->String{

    let name_station_tiempo_presente= obtain_nombre_estacion_tiempo_presente(alias_estacion);
    println!("{}",name_station_tiempo_presente);
    if let Some(reportes)= data_tiempo_presente.get(&name_station_tiempo_presente){
        let reporte = &reportes[0];
        let reporte_json = serde_json::to_string(&reporte).unwrap();
        reporte_json
    }
    else {
        "This station has not data".to_string()
    }

}

fn consult_pronostico(alias_estacion:&str, data_tiempo_presente:HashMap<String,Vec<Pronostico>>)->String{

    let name_station_tiempo_presente= obtain_nombre_estacion_pronostico(alias_estacion);
    println!("{}",name_station_tiempo_presente);
    if let Some(reportes)= data_tiempo_presente.get(&name_station_tiempo_presente){
        let reporte = &reportes[0];
        let reporte_json = serde_json::to_string(&reporte).unwrap();
        reporte_json
    }
    else {
        "This station has not data".to_string()
    }

}
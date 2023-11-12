use std::collections::HashMap;

use chrono::{Local, Duration};
use crate::extract_data::{filter_data_tiempo_presente, filter_data_pronostico, filter_data_dato_horario};
use crate::models::{TiempoPresente, Pronostico, DatoHorario};

//El link de descarga utiliza la fecha por lo que cambia cada dia


pub fn update_datos_tiempo_presente()->HashMap<String,Vec<TiempoPresente>>{
        
    let path_file_for_download= "observaciones/tiepre";
    let url_update_daily = create_url_download(path_file_for_download.to_string(),0);
    let data_from_tiempo_presente = download_data(&url_update_daily);
    
    return filter_data_tiempo_presente( data_from_tiempo_presente );
}


pub fn update_datos_pronostico()-> HashMap< String,Vec<Pronostico>>{

    let path_file_for_download = "pron5d/pron";
    let url_pronostico = create_url_download(path_file_for_download.to_string(),0);
    let data_from_pronostico = download_data(&url_pronostico);

    return filter_data_pronostico(data_from_pronostico)
}


pub fn update_datos_horarios()-> Vec<DatoHorario>{

    let path_file_for_download = "observaciones/datohorario";
    let url_datos_horarios = create_url_download(path_file_for_download.to_string(),1); //Without extension
    let data = download_data(&url_datos_horarios);

    return filter_data_dato_horario(data);
}



fn download_data(url:&str)-> String{

    let response = reqwest::blocking::get(url)
                                        .expect("No es posible actualizar la informacion");
    let body_response = response.text()
                                        .expect("No es posible acceder a la informacion descargada");
    return  body_response;
}



fn create_url_download(file: String, day_ago: i64)-> String{

    let date_today = (Local::now() - Duration::days(day_ago)).date_naive().format("%Y%m%d").to_string();
    let url = "https://ssl.smn.gob.ar/dpd/descarga_opendata.php?file=".to_string() + &file ; 


    let url_final= (url+ &date_today+".txt").to_string();
    println!("{}", url_final);

    return  url_final;
}



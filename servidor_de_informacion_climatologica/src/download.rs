use std::io;
use std::fs::File;
use chrono::Local;
use crate::extract_data::{filter_data_tiempo_presente, filter_data_pronostico};
use crate::structs::{TiempoPresente, Pronostico};

pub fn actualizar_datos_tiempo_presente()->Vec<TiempoPresente>{
        
    let name_file_for_download= "observaciones/tiepre";
    //El link de descarga utiliza la fecha por lo que cambia cada dia
    let url_update_daily = create_url_download(name_file_for_download.to_string());
    let data_from_tiempo_presente = download_data(&url_update_daily);
    
    return filter_data_tiempo_presente( data_from_tiempo_presente );

    //let name_file_for_save = "tiempo_presente";
    //create_file(name_file_for_save, data)
}


pub fn pronostico()-> Vec<Pronostico>{

    let name_file_for_download = "pron5d/pron";
    let url_pronostico = create_url_download(name_file_for_download.to_string());
    let data_from_pronostico = download_data(&url_pronostico);

    return filter_data_pronostico(data_from_pronostico)

    //let name_file_for_save = "pronostico";
    //create_file(name_file_for_save, data)
}


pub fn datos_horarios(){

    let name_file_for_download = "bservaciones/datohorario";
    let url_datos_horarios = create_url_download(name_file_for_download.to_string()); //Without extension
    let data = download_data(&url_datos_horarios);
    
    let name_file_for_save = "datos_horarios";
    create_file(name_file_for_save, data)
}



pub fn download_data(url:&str)-> String{

    let response = reqwest::blocking::get(url)
                                        .expect("No es posible actualizar la informacion");
    let body_response = response.text()
                                        .expect("No es posible acceder a la informacion descargada");
    return  body_response;
}


fn create_file(name_file:&str, body_response:String){


    let mut name_file_txt = name_file.to_owned();
    name_file_txt.push_str(".txt");

    println!("{}", name_file_txt);
    let mut archivo_salida = File::create(name_file_txt)
                                     .expect("Error al crear el archivo");
    io::copy(&mut body_response.as_bytes(), &mut archivo_salida)
                                     .expect("Error al copiar los datos en el archiv de texto");


}



fn create_url_download(file: String)-> String{

    let date_today = Local::now().date_naive().format("%Y%m%d").to_string();

    let url = "https://ssl.smn.gob.ar/dpd/descarga_opendata.php?file=".to_string() + &file ; 

    return (url+ &date_today+".txt").to_string();
}





use std::io;

use std::fs::File;




pub fn tiempo_presente(){


    let url_tiempo_presente = "https://ssl.smn.gob.ar/dpd/descarga_opendata.php?file=observaciones/tiepre20231026.txt";

    let name_file = "tiempo_presente"; //Without extension

    let data = download_data(url_tiempo_presente);


    create_file(name_file, data)


}


pub fn pronostico(){

    let url_pronostico = "https://ssl.smn.gob.ar/dpd/descarga_opendata.php?file=pron5d/pron20231026.txt";

    let name_file = "pronostico"; //Without extension

    let data = download_data(url_pronostico);


    create_file(name_file, data)

}


pub fn datos_horarios(){


    let url_datos_horarios = "https://ssl.smn.gob.ar/dpd/descarga_opendata.php?file=observaciones/datohorario20231025.txt";

    let name_file = "datos_horarios"; //Without extension

    let data = download_data(url_datos_horarios);


    create_file(name_file, data)

}



fn download_data(url:&str)-> String{

    

    let response = reqwest::blocking::get(url)

                                                .expect("No es posible actualizar la informacion");

    let body_response = response.text()

                                                .expect("No es posible acceder a la informacion descargada");

    return  body_response;


}


fn create_file(name_file:&str, body_response:String){

    

    let mut name_file_txt = name_file.to_owned();

    name_file_txt.push_str(".txt");


    let mut archivo_salida = File::create(name_file_txt)

                                     .expect("Error al crear el archivo");

    io::copy(&mut body_response.as_bytes(), &mut archivo_salida)

                                     .expect("Error al copiar los datos en el archiv de texto");


}







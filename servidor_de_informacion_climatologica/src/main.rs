
use servidor_de_informacion_climatologica:: update_data::update_datos_pronostico;






fn main() {
    
    
    
    let data = update_datos_pronostico();

    for (key, value) in data{
        println!("ID: {}, Datos: {:?}", key, value);
    }





}
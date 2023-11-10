



// #[derive(Debug)]
// struct SerieDeDatos {
//     // Puedes agregar los campos que necesites
//     datos: Vec<i32>,
// }

use servidor_de_informacion_climatologica::querys_database::{obtener_nombre_estacion_tiempo_presente, obtener_nombre_estacion_dato_horario, obtener_nombre_estacion_pronostico};





fn main() {
    
    
    // let mut series_map : HashMap<&str, SerieDeDatos> = HashMap::new();


    // let serie1 = SerieDeDatos { datos: vec![1, 2, 3] };
    // let serie2 = SerieDeDatos { datos: vec![4, 5, 6] };    
    
    // series_map.insert("id1", serie1);
    // series_map.insert("id2", serie2);
    
    
    //  // Acceder a una serie por su ID
    //  if let Some(serie) = series_map.get("id1") {
    //     println!("Datos para id1: {:?}", serie);
    // } else {
    //     println!("No se encontró la serie para id1");
    // }


    let alias_endpoint = "aeroparque";
    let presente:String = obtener_nombre_estacion_tiempo_presente(alias_endpoint);
    let horario:String = obtener_nombre_estacion_dato_horario(alias_endpoint);    
    let pronostico: String = obtener_nombre_estacion_pronostico(alias_endpoint);

    println!("{}-{}-{}",presente,horario,pronostico);
    

    // let start_time = Instant::now();

    // let reports = update_datos_horarios();
    // insert_reports(reports);

    // let end_time = Instant::now();
    // let duration_total = end_time.duration_since(start_time);
    // println!("La ejecución de todo el programa tomo {} segundos", duration_total.as_secs());


}
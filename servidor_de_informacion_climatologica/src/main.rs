
extern crate servidor_de_informacion_climatologica;
extern crate diesel;use self::servidor_de_informacion_climatologica::*;
use self::models::*;


use self::diesel::prelude::*;fn main() {
    use self::schema::ESTACIONES::dsl::*;
    
    let mut connection = establish_connection();

    let results = ESTACIONES        
        .limit(10)
        .load::<Estacion>(& mut connection)
        .expect("Error loading posts");
    println!("-----------------------------------------");
    println!("Found {} estaciones", results.len());
    println!("----------------------------------------\n");
    for p in results {
        println!("Alias : {:?}", p.alias);
        println!("Tiempo Presente : {:?}", p.nombre_estacion_tiempo_presente.unwrap());
        println!("Dato Horario : {:?}", p.nombre_estacion_dato_horario.unwrap());
        println!("Pronostico : {:?}", p.nombre_estacion_pronostico.unwrap());
        println!("----------------------------------------\n");
        
    }
}
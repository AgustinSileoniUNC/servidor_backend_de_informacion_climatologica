    
extern crate diesel;

use crate::models::{Estacion, DatoHorario};
use crate::schema::estaciones::dsl::estaciones;
use crate::schema::reportes_dato_horario::dsl::reportes_dato_horario;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;


pub fn obtain_estaciones_db(quantity :i64){
            
    let mut connection = establish_connection();

    let results = estaciones
        .limit(quantity)
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


pub fn establish_connection()-> MysqlConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
}

pub fn consult_reports(quantity :i64){

    let mut connection = establish_connection();

    let results = reportes_dato_horario
        .limit(quantity)
        .load::<DatoHorario>(& mut connection)
        .expect("Error loading posts");
    println!("-----------------------------------------");
    println!("Found {} reportes", results.len());
    println!("----------------------------------------\n");
    for p in results {
        println!("Alias : {:?}", p.estacion);
        println!("Fecha : {:?}", p.fecha);
        println!("Hora : {:?}", p.hora);
        println!("Temperatura : {:?}", p.temperatura.unwrap());
        println!("Humedad : {:?}", p.humedad_relativa.unwrap());
        println!("Presion : {:?}", p.presion_superficie.unwrap());
        println!("Viento Direccion : {:?}", p.viento_direccion.unwrap());
        println!("Viento Intensidad : {:?}", p.viento_intensidad.unwrap());
        println!("----------------------------------------\n");
        
    }
}
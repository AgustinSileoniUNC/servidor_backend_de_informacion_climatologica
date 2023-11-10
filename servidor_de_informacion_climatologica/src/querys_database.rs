    
extern crate diesel;

use crate::models::{Estacion, DatoHorario, Fecha};
use crate::schema::estaciones::dsl::estaciones;
use crate::schema::fechas;
use crate::schema::reportes_dato_horario::dsl::reportes_dato_horario;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;


fn establish_connection()-> MysqlConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
}


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



pub fn consult_reports(quantity :i64){

    let mut connection = establish_connection();

    let _ = reportes_dato_horario
        .limit(quantity)
        .load::<DatoHorario>(& mut connection)
        .expect("Error loading posts");
}


pub fn insert_reports(reports: Vec<DatoHorario>){
    
    let mut connection = establish_connection();

    _ = diesel::insert_into(reportes_dato_horario).values(&reports).execute(& mut connection);


}



pub fn fechas(){
    let mut connection = establish_connection();

    let new_date = chrono::NaiveDate::from_ymd_opt(2023, 11, 6);

    let new_time = chrono::NaiveTime::from_hms_opt(14,30,0);

    let new_row = Fecha {
        id_fecha: 1,
        fecha: new_date,
        hora: new_time.unwrap()
    };
    
    _ = diesel::insert_into(fechas::table).values( &new_row).execute(& mut connection);
}


pub fn obtener_nombre_estacion_tiempo_presente(alias_endpoint:& str)-> String{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .select( nombre_estacion_tiempo_presente.nullable())
            .filter(alias.eq(alias_endpoint))
            .load::<Option<String>>(& mut connection)
            .expect("Error loading station");

    match &results[0]{
        Some(x) => return x.to_string(),
        None => "Error loading station".to_string(),
    }

    
}


pub fn obtener_nombre_estacion_dato_horario(alias_endpoint:& str)-> String{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .select( nombre_estacion_dato_horario.nullable())
            .filter(alias.eq(alias_endpoint))
            .load::<Option<String>>(& mut connection)
            .expect("Error loading station");

    match &results[0]{
        Some(x) => return x.to_string(),
        None => "Error loading station".to_string(),
    }

    
}



pub fn obtener_nombre_estacion_pronostico(alias_endpoint:& str)-> String{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .select( nombre_estacion_pronostico.nullable())
            .filter(alias.eq(alias_endpoint))
            .load::<Option<String>>(& mut connection)
            .expect("Error loading station");

    match &results[0]{
        Some(x) => return x.to_string(),
        None => "Error loading station".to_string(),
    }

    
}
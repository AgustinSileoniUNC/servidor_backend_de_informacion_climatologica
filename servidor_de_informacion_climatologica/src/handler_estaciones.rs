
extern crate diesel;
use diesel::prelude::*;
use crate::{connection_db::establish_connection, models::Estacion};

pub fn obtain_nombre_estacion_tiempo_presente(alias_endpoint:& str)-> String{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .select( nombre_estacion_tiempo_presente.nullable())
            .filter(alias.eq(alias_endpoint))
            .load::<Option<String>>(& mut connection)
            .expect("Error loading station");

    return match &results[0]{
        Some(nombre) => nombre.to_string(),
        None => "Error loading station".to_string(),
    }
}

pub fn obtener_estacion(alias_endpoint:& str)-> Estacion{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .filter(alias.eq(alias_endpoint))
            .load::<Estacion>(& mut connection)
            .expect("Error loading station");

    let estacion = &results[0];

    return estacion.clone();
    
}


pub fn obtain_nombre_estacion_dato_horario(alias_endpoint:& str)-> String{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .select( nombre_estacion_dato_horario.nullable())
            .filter(alias.eq(alias_endpoint))
            .load::<Option<String>>(& mut connection)
            .expect("Error loading station");

    return match &results[0]{
        Some(nombre) =>   nombre.to_string(),
        None => return "Error loading station".to_string(),
    }
}


pub fn obtain_nombre_estacion_pronostico(alias_endpoint:& str)-> String{
    use crate::schema::estaciones::dsl::*;

    let mut connection = establish_connection();

    let results = estaciones
            .select( nombre_estacion_pronostico.nullable())
            .filter(alias.eq(alias_endpoint))
            .load::<Option<String>>(& mut connection)
            .expect("Error loading station");

    return match &results[0]{
        Some(nombre) => nombre.to_string(),
        None => "Error loading station".to_string(),
    }   
}


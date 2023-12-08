extern crate diesel;

use diesel::{ExpressionMethods, BoolExpressionMethods,prelude::*};
use crate::{handler_date::date_since_ago, handler_estaciones::obtain_nombre_estacion_dato_horario, schema::reportes_dato_horario::{estacion, fecha, dsl::reportes_dato_horario}, models::DatoHorario, connection_db::establish_connection};



fn consult_reports_dato_horario(alias:&str ,since_days:u64)-> Vec<DatoHorario>{

    let since_date = date_since_ago(since_days);

    let mut connection = establish_connection();

    let nombre_estacion = obtain_nombre_estacion_dato_horario(alias);

    let results = reportes_dato_horario
        .filter(fecha.ge(since_date).and(estacion.eq(nombre_estacion)))
        .load::<DatoHorario>(& mut connection)
        .expect("Error loading posts");

    return results;
    
}


pub fn consult_reports_dato_horario_5_days(alias:&str)-> Vec<DatoHorario> {

    return consult_reports_dato_horario(alias, 5 );

}

pub fn consult_reports_dato_horario_10_days(alias:&str)-> Vec<DatoHorario> {

    return consult_reports_dato_horario(alias, 10 );

}

pub fn consult_reports_dato_horario_30_days(alias:&str)-> Vec<DatoHorario> {

    return consult_reports_dato_horario(alias, 30 );

}

pub fn insert_reports(reports: Vec<DatoHorario>){
    
    let mut connection = establish_connection();

    _ = diesel::insert_into(reportes_dato_horario).values(&reports).execute(& mut connection);

}
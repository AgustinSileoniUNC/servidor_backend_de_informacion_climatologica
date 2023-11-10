// @generated automatically by Diesel CLI.

diesel::table! {
    estaciones (id_estacion) {
        id_estacion -> Integer,
        #[max_length = 45]
        alias -> Varchar,
        #[max_length = 45]
        nombre_estacion_tiempo_presente -> Nullable<Varchar>,
        #[max_length = 45]
        nombre_estacion_dato_horario -> Nullable<Varchar>,
        #[max_length = 45]
        nombre_estacion_pronostico -> Nullable<Varchar>,
    }
}

diesel::table! {
    reportes_dato_horario(id_reporte){
        id_reporte -> Integer,
        #[max_length = 45]
        estacion -> Varchar,
        fecha -> Datetime,
        #[max_length = 45]
        temperatura -> Nullable<Varchar>,
        #[max_length = 45]
        humedad_relativa -> Nullable<Varchar>,
        #[max_length = 45]
        presion_superficie -> Nullable<Varchar>,
        #[max_length = 45]
        viento_direccion -> Nullable<Varchar>,
        #[max_length = 45]
        viento_intensidad -> Nullable<Varchar>,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    ESTACIONES (id_estacion) {
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

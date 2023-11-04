use diesel::prelude::Queryable;


#[derive(Debug)]
pub struct TiempoPresente{
    pub estacion: String,
    pub fecha: String,
    pub hora: String,
    pub estado_nuboso: String,
    pub viento_intensidad: String,  
    pub temperatura: String,
    pub sensacion_termica: String,
    pub humedad_relativa: String,
    pub viento_direccion: String,
    pub presion_superficie: String
}

#[derive(Debug)]
pub struct Pronostico{
    pub estacion: String,
    pub fecha: String,
    pub hora: String,
    pub temperatura: String,
    pub viento_direccion: String,
    pub viento_intensidad: String,
    pub precipitacion: String
}

#[derive(Debug)]
pub struct DatoHorario{
    pub estacion: String,
    pub fecha: String,
    pub hora: String,
    pub temperatura: String,
    pub humedad_relativa: String,
    pub presion_superficie: String,
    pub viento_direccion: String,
    pub viento_intensidad: String,
}

#[derive(Queryable,Debug)]
pub struct Estacion{
    pub id_estacion: i32,
    pub alias :String,
    pub nombre_estacion_tiempo_presente: Option<String>,
    pub nombre_estacion_dato_horario: Option<String>,
    pub nombre_estacion_pronostico:Option<String>
}
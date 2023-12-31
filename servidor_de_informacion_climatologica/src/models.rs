use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::reportes_dato_horario;



#[derive(Debug,Clone,Serialize,Deserialize)]
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

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Pronostico{
    pub estacion: String,
    pub fecha: String,
    pub hora: String,
    pub temperatura: String,
    pub viento_direccion: String,
    pub viento_intensidad: String,
    pub precipitacion: String
}

#[derive(Queryable,Debug,Insertable,Clone)]
#[diesel(table_name= reportes_dato_horario)]
pub struct DatoHorario{
    pub id_reporte: i32,
    pub estacion: String,
    pub fecha: NaiveDateTime,
    pub temperatura: Option<String>,
    pub humedad_relativa: Option<String>,
    pub presion_superficie: Option<String>,
    pub viento_direccion: Option<String>,
    pub viento_intensidad: Option<String>,
}

#[derive(Queryable,Debug,Serialize,Deserialize,Clone)]
pub struct Estacion{
    pub id_estacion: i32,
    pub alias :String,
    pub nombre_estacion_tiempo_presente: Option<String>,
    pub nombre_estacion_dato_horario: Option<String>,
    pub nombre_estacion_pronostico:Option<String>
}


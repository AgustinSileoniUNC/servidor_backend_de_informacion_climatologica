
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

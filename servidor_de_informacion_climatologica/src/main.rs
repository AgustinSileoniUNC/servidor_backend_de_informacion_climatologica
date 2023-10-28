use servidor_de_informacion_climatologica::download::actualizar_datos_tiempo_presente;



fn main() {

    let  data_tiempo_presente= actualizar_datos_tiempo_presente();


    for registro in &data_tiempo_presente{
        println!("Estacion: {} - Hora: {} - Temperatura: {} - Humedad: {}", registro.estacion, registro.hora, registro.temperatura, registro.humedad_relativa)
    }

}

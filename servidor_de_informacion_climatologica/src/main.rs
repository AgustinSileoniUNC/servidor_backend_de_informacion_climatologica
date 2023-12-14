#[allow(dead_code)]
use std::collections::HashMap;
use std::{sync::RwLock, time::Instant};
use servidor_de_informacion_climatologica::{update_data::{update_datos_tiempo_presente, update_datos_pronostico, update_datos_horarios}, handler_reports_dato_horario::insert_reports};
#[allow(dead_code)]
use servidor_de_informacion_climatologica::{handler_estaciones::{obtain_nombre_estacion_tiempo_presente, obtain_nombre_estacion_pronostico}, models::{TiempoPresente, Pronostico}, thread_pool::ThreadPool};
#[allow(dead_code)]
use std::{thread::{self}, net::{TcpListener, TcpStream}, io::{Read, Write}, time::Duration, fs};

#[allow(dead_code)]
fn main() {

    let start_time = Instant::now();

    let data_pronostico = RwLock::new(HashMap::<String,Vec<Pronostico>>::new());
    let data_tiempo_presente = RwLock::new(HashMap::<String, Vec<TiempoPresente>>::new());

    let data_pronostico = thread::spawn(move ||{
        let start_time_thread_pronostico = Instant::now();

        let auxiliar = update_datos_pronostico();
        
        let mut lock_data_pronostico = data_pronostico.write().unwrap();
        *lock_data_pronostico = auxiliar;
        drop(lock_data_pronostico);

        let end_time_pronostico = Instant::now();
        let elapsed_time = (end_time_pronostico - start_time_thread_pronostico).as_secs_f64();
        println!("Tiempo de ejecución pronostico: {} segundos", elapsed_time);

        return data_pronostico;
    });

    let data_tiempo_presente = thread::spawn(move ||{
        let start_time_thread_tiempo_presente = Instant::now();
        
        let auxiliar = update_datos_tiempo_presente();

        let mut lock_data_tiempo_presente = data_tiempo_presente.write().unwrap();
        *lock_data_tiempo_presente = auxiliar;        
        drop(lock_data_tiempo_presente);

        let end_time_tiempo_presente = Instant::now();
        let elapsed_time = (end_time_tiempo_presente - start_time_thread_tiempo_presente).as_secs_f64();
        println!("Tiempo de ejecución tiempo presente: {} segundos", elapsed_time);

        return data_tiempo_presente;
    });

    let _ = thread::spawn(||{
        let start_time_thread_datos_horarios = Instant::now();
        insert_reports(update_datos_horarios());
        let end_time_datos_horarios = Instant::now();
        let elapsed_time = (end_time_datos_horarios - start_time_thread_datos_horarios).as_secs_f64();
        println!("Tiempo de ejecución datos_horarios: {} segundos", elapsed_time);
    });


    let data_pronostico = data_pronostico.join();
    let data_tiempo_presente = data_tiempo_presente.join();

    let binding_pronostico = data_pronostico.unwrap();
    let lectura_pronostico = binding_pronostico.read().unwrap();
     let pronostico = consult_pronostico("azul", &*lectura_pronostico);
     println!("Pronostico: {}", pronostico);
    drop(lectura_pronostico);
    
    println!("Hilo Pronostico");


    let binding_tiempo_presente = data_tiempo_presente.unwrap();
    let lectura_tiempo_presente = binding_tiempo_presente.read().unwrap();
    let tiempo_presente = consult_tiempo_presente("concordia",&*&lectura_tiempo_presente);
    println!("Tiempo Presente: {}", tiempo_presente);
    drop(lectura_tiempo_presente);

    println!("Hilo Tiempo Presente");

    let end_time = Instant::now();
    let elapsed_time = (end_time - start_time).as_secs_f64();
    println!("Tiempo de ejecución: {} segundos", elapsed_time);

    
    
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //Los hilos se crean en el ThreadPool y aca solo se arrancan
        pool.execute(move || {
            handle_connection(stream)
        });
        
    }



}


#[allow(dead_code)]
fn handle_connection(mut stream: TcpStream){

    let mut buffer =[0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename)=
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        }
        else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        else{
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

  
}



#[allow(dead_code)]
fn consult_tiempo_presente(alias_estacion:&str, data_tiempo_presente:&HashMap<String,Vec<TiempoPresente>>)->String{

    let name_station_tiempo_presente= obtain_nombre_estacion_tiempo_presente(alias_estacion);
    println!("{}",name_station_tiempo_presente);
    if let Some(reportes)= data_tiempo_presente.get(&name_station_tiempo_presente){
        let reporte = &reportes[0];
        let reporte_json = serde_json::to_string(&reporte).unwrap();
        reporte_json
    }
    else {
        "This station has not data".to_string()
    }

}

#[allow(dead_code)]
fn consult_pronostico(alias_estacion:&str, data_tiempo_presente:&HashMap<String,Vec<Pronostico>>)->String{

    let name_station_tiempo_presente= obtain_nombre_estacion_pronostico(alias_estacion);
    println!("{}",name_station_tiempo_presente);
    if let Some(reportes)= data_tiempo_presente.get(&name_station_tiempo_presente){
        let reporte = &reportes[0];
        let reporte_json = serde_json::to_string(&reporte).unwrap();
        reporte_json
    }
    else {
        "This station has not data".to_string()
    }

}
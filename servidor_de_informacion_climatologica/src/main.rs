use std::collections::HashMap;

use servidor_de_informacion_climatologica::{handler_estaciones::{obtain_nombre_estacion_tiempo_presente, obtain_nombre_estacion_pronostico}, models::{TiempoPresente, Pronostico}};

use std::thread;
use std::time;


fn main() {


    let mut vec_tiempo_presente : Vec<i64> = Vec::new();
    let mut vec_pronostico : Vec<i64> = Vec::new();
    


    let handle = thread::spawn(||{
        let mut auxiliar_tiempo_presente : Vec<i64>= Vec::new();
        for i in 1..100{
            println!("{}",i);
            auxiliar_tiempo_presente.push(i);
        }
        return auxiliar_tiempo_presente;
    });
    let handle2 = thread::spawn(||{
        let mut auxiliar_pronostico : Vec<i64>= Vec::new();
        for i in 100..200{
            println!("{}",i);
            auxiliar_pronostico.push(i);
        }
        return auxiliar_pronostico;
    });

    let req = thread::spawn(||{
        println!("Consultandoo....");
        thread::sleep(time::Duration::from_millis(3000));
        println!("Respondido");
    });


    for n in 300..400{
        println!("{}",n);
    }

    vec_tiempo_presente= handle.join().unwrap();
    vec_pronostico = handle2.join().unwrap();
    
    let _ = req.join();
    println!("Hilo finalizado");

    for n in vec_tiempo_presente{
        println!("Tiempo Presente {:?}", n);
    }

    for n in vec_pronostico{
        println!("Pronostico {:?}", n);
    }


    



    
    
//     let data_tiempo_presente:HashMap<String, Vec<TiempoPresente>>;
//     let  datos_pronostico: HashMap<String,Vec<Pronostico>>;

//     data_tiempo_presente =  update_datos_tiempo_presente();
//     datos_pronostico = update_datos_pronostico();


//    let result = consult_tiempo_presente("concordia", data_tiempo_presente);

//     println!("{}",result);

//     let pronostico = consult_pronostico("concordia", datos_pronostico);
    
//     println!("{}", pronostico);


    






}







fn consult_tiempo_presente(alias_estacion:&str, data_tiempo_presente:HashMap<String,Vec<TiempoPresente>>)->String{

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

fn consult_pronostico(alias_estacion:&str, data_tiempo_presente:HashMap<String,Vec<Pronostico>>)->String{

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
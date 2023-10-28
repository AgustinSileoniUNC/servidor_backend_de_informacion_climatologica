
use crate::{regular_expresions::obtener_estructura_tiempo_presente, structs::TiempoPresente};




pub fn obtener_datos_tiempo_presente(data:String)-> Vec<TiempoPresente>{

    let mut datos_tiempo_presente_estructura: Vec<TiempoPresente> = Vec::new();
    
    let lines = data.lines();


    for (_ , line) in lines.enumerate() {
        let valor = obtener_estructura_tiempo_presente(evite_empty_values(line.to_string()));
        //println!("{} - {:?}", number, valor);
        datos_tiempo_presente_estructura.push( valor );
    }

    return datos_tiempo_presente_estructura;
}


fn evite_empty_values(original_text_line:String)-> String{

    let mut text_line= original_text_line.to_string();

    if original_text_line.contains(";;"){
        text_line = original_text_line.replace(";;", ";0 km;");
    }

    return text_line.to_string();

}
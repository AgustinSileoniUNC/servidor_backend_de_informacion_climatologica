
use std::collections::BTreeMap;

use servidor_de_informacion_climatologica::update_data::update_datos_tiempo_presente;






fn main() {
    
    
    let data = update_datos_tiempo_presente();

    // Obtén las claves y ordénalas alfabéticamente
    let claves_ordenadas: BTreeMap<_, _> = data.into_iter().collect();

    // Imprime las claves ordenadas
    for clave in claves_ordenadas.keys() {
        println!("{}", clave);
    }



}
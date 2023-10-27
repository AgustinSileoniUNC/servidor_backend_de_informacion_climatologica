use servidor_de_informacion_climatologica::regular_expresions;





fn main() {

    let texto = "Azul;27-Octubre-2023;15:00;Parcialmente nublado;15 km;24.8;No se calcula; 47;Oeste  14;985 / ";
    let afuera = regular_expresions::obtener_tiempo_presente(texto.to_string());

    println!("Tiempo Presente {:?}", afuera.presion_superficie);



}

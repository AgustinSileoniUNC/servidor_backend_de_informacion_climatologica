use servidor_de_informacion_climatologica::handler_reports_dato_horario::consult_reports_dato_horario_5_days;




fn main() {
    
    
    let results = consult_reports_dato_horario_5_days("azul");

    for result in results{
        println!("{:?}", result)
    }



}
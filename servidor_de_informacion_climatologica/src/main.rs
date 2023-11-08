use servidor_de_informacion_climatologica::{update_data:: update_datos_horarios, querys_database::insert_reports};



    




fn main() {
    
    insert_reports(update_datos_horarios());

}
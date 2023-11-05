use servidor_de_informacion_climatologica::{querys_database::insert_reports, download_data::update_datos_horarios};








fn main() {
    

    insert_reports(update_datos_horarios());

}
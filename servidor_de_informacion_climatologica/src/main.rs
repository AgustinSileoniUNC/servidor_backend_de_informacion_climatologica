use servidor_de_informacion_climatologica::download;



fn main() {

    let informes =download::pronostico();

    println!("{:?}", informes[30]);

}

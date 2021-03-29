mod centros;
mod utilities;

use std::error::Error;
use std::process;

use centros::Economy;

fn main() {

    if let Err(err) = random_grid() {
        println!("{}", err);
        process::exit(1);
    }
}

fn random_grid() -> Result<(), Box<dyn Error>> {

    const X_MAX: usize = 50;
    const Y_MAX: usize = 50;
    const POBLACION: f64 = 1.0;
    const CENTROS: usize = 20;
    const ITERACIONES: usize = 200;

    let mut celdas = utilities::grid_of_cells(X_MAX, Y_MAX, POBLACION);

    let proto_sectores = vec![("sector_1",1.0),("sector_2",3.0)];
    let sectores = utilities::sectors_from_vec(proto_sectores);
    for (_, sector) in sectores.iter() {
        utilities::define_random_centers(CENTROS, &mut celdas, sector);
    }
    
    utilities::escribir_topologia(&celdas, "./salida/celdas.csv")?;

    let directorio = "./salida/";
    let mut salida = utilities::get_salida(&sectores, &celdas, directorio)?;

    for t in 0..ITERACIONES {
        if t % 50 == 0 {
            println!("t = {}", t)
        }

        celdas.evolve(&sectores);
        utilities::escribir_iteracion(&mut salida, &celdas)?;
    }

    utilities::flush_salida(&mut salida)?;

    Ok(())
}



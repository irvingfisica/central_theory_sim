mod centros;
mod utilities;

use std::error::Error;
use std::process;
// use std::fs::File;
use std::collections::HashMap;

use centros::Economy;
use centros::{Sector, Celda};

fn main() {

    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {

    const X_MAX: usize = 50;
    const Y_MAX: usize = 50;
    const POBLACION: f64 = 1.0;
    const CENTROS: usize = 20;
    const ITERACIONES: usize = 200;

    let proto_sectores = vec![("sector_1",1.0)];
    let sectores = utilities::sectors_from_vec(proto_sectores);
    let sector = sectores.get("sector_1").ok_or("El sector no existe")?;

    let mut celdas = utilities::grid_of_cells(X_MAX, Y_MAX, POBLACION);

    let _cves = utilities::define_centers(CENTROS, &mut celdas, sector);

    for t in 0..ITERACIONES {
        if t % 50 == 0 {
            println!("t = {}", t)
        }

        celdas.evolve(&sectores);
    }

    Ok(())
}


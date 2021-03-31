mod centros;
mod utilities;

use std::error::Error;
use std::process;

use easytiming::Timing;
use std::io::Stdout;

use centros::Economy;

fn main() {

    if let Err(err) = agebs() {
        println!("{}", err);
        process::exit(1);
    }
}

fn agebs() -> Result<(), Box<dyn Error>> {
    const ITERACIONES: usize = 200;

    let mut celdas = utilities::topo_from_file("./datos/procesados/agebs_cdmx_pob.csv")?;

    let proto_sectores = vec![
            ("cm_0p50",0.5),
            ("cm_0p75",0.75),
            ("cm_1p00",1.0),
            ("cm_1p25",1.25),
            ("cm_1p50",1.5),
            ("cm_1p75",1.75),
            ("cm_2p00",2.0),
            ("cm_2p25",2.25),
            ("cm_2p50",2.5),
            ("cm_2p75",2.75),
        ];

    let sectores = utilities::sectors_from_vec(proto_sectores);
    // let sector = sectores.get("cemp_mpio_3").expect("El sector no existe");

    for (_,sector) in sectores.iter() {
        utilities::centers_from_file("./datos/procesados/cemp_cdmx.csv", &mut celdas, sector)?;
    }
    
    utilities::escribir_topologia(&celdas, "./salida/celdas_agebs.csv")?;

    let directorio = "./salida/";
    let mut salida = utilities::get_salida(&sectores, &celdas, directorio)?;

    for _ in 0..ITERACIONES {
        {
        let _t : easytiming::Timing<'_, Stdout>  = Timing::new("test() function");
        // if t % 50 == 0 {
        //     println!("t = {}", t)
        // }

        celdas.evolve(&sectores);
        utilities::escribir_iteracion(&mut salida, &celdas)?;
        }
    }

    utilities::flush_salida(&mut salida)?;

    Ok(())
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



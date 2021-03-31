mod centros;
mod utilities;

use std::error::Error;
use std::process;

// use easytiming::Timing;
// use std::io::Stdout;

use centros::Economy;

fn main() {

    if let Err(err) = random_ensamble() {
        println!("{}", err);
        process::exit(1);
    }
}

fn agebs() -> Result<(), Box<dyn Error>> {
    const ITERACIONES: usize = 200;

    let mut celdas = utilities::topo_from_file("./datos/procesados/agebs_cdmx_pob.csv")?;

    let proto_sectores = vec![
            (String::from("rs_1p00"),1.0),
            (String::from("rs_1p50"),1.5),
            (String::from("rs_2p00"),2.0),
            (String::from("rs_2p75"),2.75),
        ];

    let sectores = utilities::sectors_from_vec(proto_sectores);
    // let sector = sectores.get("cemp_mpio_3").expect("El sector no existe");

    for (_,sector) in sectores.iter() {
        utilities::centers_from_file("./datos/procesados/rests_cdmx.csv", &mut celdas, sector)?;
    }
    
    utilities::escribir_topologia(&celdas, "./salida/celdas_agebs.csv")?;

    let directorio = "./salida/";
    let mut salida = utilities::get_salida(&sectores, &celdas, directorio)?;

    for t in 0..ITERACIONES {
        {
        // let _t : easytiming::Timing<'_, Stdout>  = Timing::new("test() function");
        // if t % 50 == 0 {
            println!("t = {}", t);
        // }

        celdas.evolve(&sectores);
        utilities::escribir_iteracion(&mut salida, &celdas)?;
        }
    }

    utilities::flush_salida(&mut salida)?;

    Ok(())
}

fn random_ensamble() -> Result<(), Box<dyn Error>> {

    const X_MAX: usize = 50;
    const Y_MAX: usize = 50;
    const POBLACION: f64 = 1.0;
    const CENTROS: usize = 20;
    const ITERACIONES: usize = 200;
    const INSTANCIAS: usize = 10;

    for i in 0..INSTANCIAS {
        let mut celdas = utilities::grid_of_cells(X_MAX, Y_MAX, POBLACION);

        let istr = (i + 1).to_string();
        let mut cadena = String::from("i_");
        cadena.push_str(&istr);
        cadena.push_str("_e_");

        let proto_sectores: Vec<(String,f64)> = (0..30).map(|ent| {
            let eta = 0.1 + 0.1 * ent as f64;
            let streta = format!("{:.1}", eta);
            let mut cad = cadena.to_owned();
            cad.push_str(&streta);
            let salstr = cad.replace(".","p");
            (salstr,eta)
        }).collect();

        let sectores = utilities::sectors_from_vec(proto_sectores);
        for (_, sector) in sectores.iter() {
            utilities::define_random_centers(CENTROS, &mut celdas, sector);
        }
    
        utilities::escribir_topologia(&celdas, "./salida/ensamble_random/celdas.csv")?;

        let directorio = "./salida/ensamble_random/";
        let mut salida = utilities::get_salida(&sectores, &celdas, directorio)?;

        for t in 0..ITERACIONES {
            if t % 50 == 0 {
                println!("i = {}, t = {}", i, t)
            }

            // println!("i = {}, t = {}", i, t);

            celdas.evolve(&sectores);
            utilities::escribir_iteracion(&mut salida, &celdas)?;
        }

        utilities::flush_salida(&mut salida)?;

    }

    Ok(())

}


fn random_grid() -> Result<(), Box<dyn Error>> {

    const X_MAX: usize = 50;
    const Y_MAX: usize = 50;
    const POBLACION: f64 = 1.0;
    const CENTROS: usize = 20;
    const ITERACIONES: usize = 200;

    let mut celdas = utilities::grid_of_cells(X_MAX, Y_MAX, POBLACION);

    let proto_sectores = vec![(String::from("sector_1"),1.0),(String::from("sector_2"),3.0)];
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



mod centros;

use std::error::Error;
use std::process;
// use std::fs::File;
use std::collections::HashMap;

use rand::prelude::*;
// use easytiming::Timing;
// use std::io::Stdout;

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
    const CENTERS: usize = 20;
    const ITERACIONES: usize = 200;

    let mut sectores = HashMap::new();
    sectores.insert(String::from("sector_1"),Sector::new("sector_1", 1.0));

    let sector = sectores.get("sector_1").unwrap();

    let mut celdas: HashMap<String, Celda> = HashMap::new();

    for x in 0..X_MAX {
        for y in 0..Y_MAX {

            let mut cve = format!("{:04}", x);
            let cvey = format!("{:04}", y);
            cve.push_str(&cvey);

            let celda = Celda::new(&cve, x as f64, y as f64, 1.0);
            celdas.insert(cve,celda);
        }
    }

    let mut rng = &mut rand::thread_rng();

    let centers: usize = CENTERS.min(celdas.len());
    let cves = celdas.iter().map(|(cve,_)| cve.to_owned()).choose_multiple(&mut rng, centers);

    for cve in cves.iter() {
        celdas.get_mut(cve).unwrap().populate(0.0);
        celdas.get_mut(cve).unwrap().add_activity(&sector, 1.0, 0.5);
    };

    for cve in cves.iter() {
        let size = celdas.get(cve).unwrap().size_of_activity(&sector).unwrap();
        let pop = celdas.get(cve).unwrap().population();
        println!("cve: {}, poblacion: {}, size: {}", cve, pop, size);
    }

    for t in 0..ITERACIONES {
        {
        // let _t : easytiming::Timing<'_, Stdout>  = Timing::new("test() function");
        if t % 50 == 0 {
            println!("t = {}", t)
        }

        celdas.evolve(&sectores);
        }
    }

    for cve in cves.iter() {
        let size = celdas.get(cve).unwrap().size_of_activity(&sector).unwrap();
        let pop = celdas.get(cve).unwrap().population();
        println!("cve: {}, poblacion: {}, size: {}", cve, pop, size);
    }

    Ok(())
}


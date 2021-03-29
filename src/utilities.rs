use std::collections::HashMap;
use crate::centros::{Sector, Celda};

pub fn sectors_from_vec(sectores: Vec<(&str, f64)>) -> HashMap<String, Sector> {
    let mut mapa = HashMap::new();

    for (sector,eta) in sectores {
        mapa.insert(String::from(sector),Sector::new(sector, eta));
    };

    mapa
}

pub fn grid_of_cells<'a>(x_max: usize, y_max: usize, population: f64) -> HashMap<String, Celda<'a>> {

    let mut celdas: HashMap<String, Celda> = HashMap::new();

    for x in 0..x_max {
        for y in 0..y_max {

            let mut cve = format!("{:04}", x);
            let cvey = format!("{:04}", y);
            cve.push_str(&cvey);

            let celda = Celda::new(&cve, x as f64, y as f64, population);
            celdas.insert(cve,celda);
        }
    };

    celdas
}

pub fn define_centers<'a>(centros: usize, celdas: &mut HashMap<String,Celda<'a>>, sector: &'a Sector) -> Vec<String> {
    use rand::prelude::*;

    let mut rng = &mut rand::thread_rng();

    let centers = centros.min(celdas.len());
    let cves = celdas.iter().map(|(cve,_)| cve.to_owned()).choose_multiple(&mut rng, centers);

    let initial_size = 1.0;
    let growth_factor = 0.5;

    for cve in cves.iter() {
        celdas.get_mut(cve).unwrap().populate(0.0);
        celdas.get_mut(cve).unwrap().add_activity(sector, initial_size, growth_factor);
    };

    cves
}
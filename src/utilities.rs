use std::collections::HashMap;
use crate::centros::{Sector, Celda};
use std::fs::File;
use std::error::Error;

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

pub fn define_random_centers<'a>(centros: usize, celdas: &mut HashMap<String,Celda<'a>>, sector: &'a Sector) -> Vec<String> {
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

pub struct SalidaSector<'a> {
    sector: &'a Sector,
    centros: Vec<String>,
    writer: csv::Writer<File>,
}

impl<'a> SalidaSector<'a> {
    pub fn get_salida_sector(sector: &'a Sector, celdas: &HashMap<String, Celda>, ruta: &str) -> Result<SalidaSector<'a>,Box<dyn Error>> {

        let cves: Vec<String> = celdas.iter().filter_map(|(cve, celda)| {
            match celda.get_activity(&sector) {
                Some(_) => Some(cve.to_owned()),
                _ => None
            }
        }).collect();
    
        let writer = csv::Writer::from_path(ruta)?;
    
        let mut salida = SalidaSector {
            sector: sector,
            centros: cves,
            writer: writer,
        };
    
        salida.writer.write_record(&salida.centros)?;
    
        Ok(salida)
    }

    pub fn escribir_registro(&mut self, celdas: &HashMap<String, Celda>) -> Result<(), Box<dyn Error>> {

        let sizes: Vec<String> = self.centros.iter().map(|centro| {
            celdas.get(centro).unwrap().size_of_activity(self.sector).unwrap().to_string()
        }).collect();

        self.writer.write_record(&sizes)?;

        Ok(())
    }

    pub fn flush_writer(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.flush()?;

        Ok(())
    }
}

pub fn get_salida<'a>(sectores: &'a HashMap<String, Sector>, celdas: &HashMap<String, Celda>, directorio: &str) 
    -> Result<HashMap<String, SalidaSector<'a>>, Box<dyn Error>> {

        let mut salida = HashMap::new();

        for (cve,sector) in sectores {

            let mut ruta = String::from(directorio);
            ruta.push_str(&cve);
            ruta.push_str(".csv");

            salida.insert(cve.to_owned(),SalidaSector::get_salida_sector(sector, celdas, &ruta)?);

        };

        Ok(salida)
}

pub fn escribir_iteracion(salida: &mut HashMap<String, SalidaSector>, celdas: &HashMap<String, Celda>) 
    -> Result<(), Box<dyn Error>> {

        for (_, salida_sector) in salida {
            salida_sector.escribir_registro(celdas)?;
        };

        Ok(())
    }
    
pub fn flush_salida(salida: &mut HashMap<String, SalidaSector>) -> Result<(), Box<dyn Error>> {

    for (_, salida_sector) in salida {
        salida_sector.flush_writer()?;
    };

    Ok(())
}

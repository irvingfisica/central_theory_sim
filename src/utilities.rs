use std::collections::HashMap;
use crate::centros::{Sector, Celda};
use std::fs::File;
use std::error::Error;

pub fn sectors_from_vec(sectores: Vec<(String, f64)>) -> HashMap<String, Sector> {
    let mut mapa = HashMap::new();

    for (sector,eta) in sectores.iter() {
        mapa.insert(String::from(sector),Sector::new(&sector, *eta));
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

pub fn topo_from_file<'a>(path: &str) -> Result<HashMap<String, Celda<'a>>,Box<dyn Error>> {

    let mut celdas: HashMap<String, Celda> = HashMap::new();

    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        
        let cve = &record[0];
        let x = match record[1].parse::<f64>() {
            Ok(x) => x,
            _ => continue,
        };
        let y = match record[2].parse::<f64>() {
            Ok(x) => x,
            _ => continue,
        };
        let pob = match record[3].parse::<f64>() {
            Ok(x) => x,
            _ => continue,
        };

        let celda = Celda::new(&cve, x, y, pob);
        celdas.insert(cve.to_owned(),celda);
    }

    Ok(celdas)

}

pub fn centers_from_file<'a>(path: &str, celdas: &mut HashMap<String,Celda<'a>>, sector: &'a Sector) -> Result<Vec<String>,Box<dyn Error>> {

    let mut rdr = csv::Reader::from_path(path)?;
    let mut salida = Vec::new();

    let growth_factor = 0.5;

    // let mut cta = 0;
    for result in rdr.records() {
        let record = result?;
        
        let cve = &record[0];
        let size = match record[1].parse::<f64>() {
            Ok(x) => x,
            _ => continue,
        };

        match celdas.get_mut(cve) {
            Some(celda) => {
                celda.add_activity(sector, size, growth_factor);
                salida.push(cve.to_owned());
                // cta = cta + 1;
            },
            None => continue
        };

        // if cta >= 20 {
        //     break
        // }

    }

    Ok(salida)

}

pub fn random_vec_of_cves<'a>(centros: usize, celdas: &HashMap<String,Celda<'a>>) -> Vec<String> {
    use rand::prelude::*;

    let mut rng = &mut rand::thread_rng();

    let centers = centros.min(celdas.len());
    let cves = celdas.iter().map(|(cve,_)| cve.to_owned()).choose_multiple(&mut rng, centers);

    cves
}

pub fn centers_from_vec<'a>(cves: &Vec<String>, size: f64, celdas: &mut HashMap<String,Celda<'a>>, sector: &'a Sector) -> Result<Vec<String>,Box<dyn Error>> {

    let growth_factor = 0.5;

    for cve in cves.iter() {

        match celdas.get_mut(cve) {
            Some(celda) => {
                celda.add_activity(sector, size, growth_factor);
            },
            None => continue
        };

    }

    Ok(cves.clone())

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

pub fn escribir_topologia(celdas: &HashMap<String, Celda>, ruta: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr_cells = csv::Writer::from_path(ruta)?;

    wtr_cells.write_record(&["CVE", "x", "y", "poblacion"])?;

    for (_,cell) in celdas.iter() {
        let coords = cell.coordinates();
        wtr_cells.write_record(&[
            &cell.cve(), 
            &coords.0.to_string(), 
            &coords.1.to_string(), 
            &cell.population().to_string()
        ])?;
    };

    wtr_cells.flush()?;

    Ok(())
}
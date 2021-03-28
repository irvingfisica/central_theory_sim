use std::collections::HashMap;
use std::error::Error;


pub struct Celda<'a> {
    cve: String,
    x: f64,
    y: f64,
    actividades: HashMap<String, Actividad<'a>>,
    poblacion: f64,
}

impl<'a> Celda<'a> {

    pub fn new(cve: &str, x: f64, y: f64, poblacion: f64) -> Self {
        
        Celda {
            cve: String::from(cve),
            x: x,
            y: y,
            actividades: HashMap::new(),
            poblacion: poblacion,
        }
    }

    pub fn add_activity(&mut self, sector: &'a Sector, size: f64, growth_factor: f64) {

        let mut actividad = Actividad::new(sector, size, growth_factor);
        let sector_cve = sector.cve.to_owned();

        match self.actividades.get(&sector_cve) {
            None => {self.actividades.insert(sector_cve, actividad);},
            Some(old_activ) => {
                actividad.size = old_activ.size + actividad.size;
                actividad.growth_factor = (old_activ.growth_factor + actividad.growth_factor) / 2.0;
                self.actividades.insert(sector_cve,actividad);
            }
        }
    }

    pub fn distance(&self, other: &Celda) -> f64 {
        let parte_x = (self.x - other.x) * (self.x - other.x);
        let parte_y = (self.y - other.y) * (self.y - other.y);

        (parte_x + parte_y).sqrt()
    }

    pub fn populate(&mut self, population: f64) {
        self.poblacion = population;
    }

    pub fn population(&self) -> f64 {
        self.poblacion 
    }

    pub fn size_of_activity(&self, sector: &'a Sector) -> Result<f64, Box<dyn Error>> {

        match self.actividades.get(&sector.cve) {
            Some(actividad) => {
                Ok(actividad.size())
            },
            None => return Err(From::from("La celda no tiene actividad para ese sector"))
        }
    }
}

pub struct Actividad<'a> {
    sector: &'a Sector,
    size: f64,
    growth_factor: f64,
}

impl<'a> Actividad<'a> {
    pub fn new(sector: &'a Sector, size: f64, growth_factor: f64) -> Self {
        Actividad {
            sector: sector,
            size: size,
            growth_factor: growth_factor,
        }
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn get_sector(&self) -> &'a Sector {
        self.sector
    }
}

pub struct Sector {
    cve: String,
    pop_param: f64,
    fixed_cost: f64,
    returns_const: f64,
    cost_exp: f64,
    p_capita_expenditure: f64,
    interaction: f64,
}

impl Sector {
    pub fn new(cve: &str, interaction: f64) -> Self {
        Sector {
            cve: String::from(cve),
            interaction: interaction,
            ..Default::default()
        }
    }
}

impl Default for Sector {
    fn default() -> Sector {
        Sector {
            cve: String::from("nameless"),
            pop_param: 0.0,
            fixed_cost: 0.0,
            returns_const: 1.0,
            cost_exp: 1.0,
            p_capita_expenditure: 1.0,
            interaction: 1.0,
        }
    }
}

pub trait Economy<T> {
    fn member_revenue(&self, elem: &T, sector: &Sector) -> Result<f64,Box<dyn Error>>;
    fn member_cost(&self, elem: &T, sector: &Sector) -> Result<f64,Box<dyn Error>>;
    fn member_size(&self, elem: &T, sector: &Sector) -> Result<f64, Box<dyn Error>>;

    fn update_sector_sizes(&mut self, sector: &Sector);
    fn update_populations(&mut self);

    fn evolve(&mut self, sectores: &HashMap<String, Sector>);
}

impl<'a> Economy<Celda<'a>> for HashMap<String, Celda<'a>> {
    fn member_revenue(&self, celda: &Celda<'a>, sector: &Sector) -> Result<f64, Box<dyn Error>> {
        
        use rayon::prelude::*;

        let actividad = match celda.actividades.get(&sector.cve) {
            Some(actividad) => {
                actividad
            },
            None => return Err(From::from("La celda no tiene actividad para ese sector"))
        };

        let revenue: f64 = self.par_iter().filter(|(_, cellxy)| cellxy.distance(celda) != 0.0 )
        .map(|(_, cellxy)| {
            
            let numer = actividad.size * cellxy.distance(celda).powf(-1.0 * sector.interaction);
            
            let denom: f64 = self.iter().filter_map(|(_, cell)| {
                match &cell.actividades.is_empty() {
                    true => None,
                    false => match &cell.actividades.get(&sector.cve) {
                        Some(activcxy) => {
                            Some(activcxy.size * cell.distance(cellxy).powf(-1.0 * sector.interaction))
                        },
                        None => None
                    }
                }
            }).sum();

            let flux = numer/denom;
            let population = cellxy.poblacion;

            flux*population

        }).sum();

        Ok(revenue * sector.p_capita_expenditure)
    }

    fn member_cost(&self, celda: &Celda<'a>, sector: &Sector) -> Result<f64, Box<dyn Error>> {

        let actividad = match celda.actividades.get(&sector.cve) {
            Some(actividad) => {
                actividad
            },
            None => return Err(From::from("La celda no tiene actividad para ese sector"))
        };

        let costo = sector.fixed_cost + sector.returns_const * (actividad.size.powf(sector.cost_exp));

        Ok(costo)

    }

    fn member_size(&self, celda: &Celda<'a>, sector: &Sector) -> Result<f64, Box<dyn Error>> {

        let actividad = match celda.actividades.get(&sector.cve) {
            Some(actividad) => {
                actividad
            },
            None => return Err(From::from("La celda no tiene actividad para ese sector"))
        };

        let revenue = self.member_revenue(&celda, &sector)?;
        let cost = self.member_cost(&celda, &sector)?;
        let margen = revenue - cost;
        let size = actividad.size + (actividad.growth_factor * margen);

        Ok(size)
        
    }

    fn update_sector_sizes(&mut self, sector: &Sector) {

        let mut mapa = HashMap::new();

        for (cve, cell) in self.iter()
            .filter(|(_, cell)| cell.actividades.get(&sector.cve).is_some()) {
            
            match self.member_size(cell, sector) {
                Ok(size) => {
                    mapa.insert(cve.to_owned(),size);
                },
                _ => {}
            }
            
        }

        for (cve, size) in mapa {
            self.get_mut(&cve).unwrap().actividades.get_mut(&sector.cve).unwrap().size = size
        }
    }

    fn update_populations(&mut self) {
        
        for (_, celda) in self.iter_mut().filter(|(_, cell)| {
            !cell.actividades.is_empty()
        }) {
            celda.poblacion = celda.actividades.iter().map(|(_, actividad)| {
                    let size = actividad.size();
                    let sector = actividad.get_sector();
                    sector.pop_param * size
                }).sum();
        }
    }

    fn evolve(&mut self, sectores: &HashMap<String, Sector>) {

        for (_,sector) in sectores.iter() {
            self.update_sector_sizes(sector);
        };

        self.update_populations();

    }

}

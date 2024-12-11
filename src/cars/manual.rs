use crate::components::{CarType, Engine, GpsNavigator, Transmission};
use std::fmt::Display;

pub struct Manual {
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GpsNavigator>,
}

impl Manual {
    pub fn new(
        car_type: CarType,
        seats: u16,
        engine: Engine,
        transmission: Transmission,
        gps_navigator: Option<GpsNavigator>,
    ) -> Self {
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
        }
    }
}

impl Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Type of car : {?}", self.car_type)?;
        writeln!(f, "counts of seats: {}", self.seats);
        writeln!(
            f,
            "Engine: Volume - {} mileage - {}",
            self.engine.volume(),
            self.engine.mileage()
        )?;

        writeln!(f, "Transmission : {:?}", self.transmission)?;
        match self.gps_navigator {
            Some(_) => writeln!(f, "GPS GpsNavigator:Functional")?,
            None => writeln!(f, "GPS Navigator: N/A"),
        };
        Ok(())
    }
}

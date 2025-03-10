use std::{cmp::Ordering, fmt::Display};

use hashbrown::HashMap;

// We don't need to use BTreeMap because we don't require sorting or updating data frequently.
// We only need to store the data and sort it once.

// It is good practice to implement common traits for custom types. :)
#[derive(Debug, Clone)]
pub struct CitiesWeather {
    entries: HashMap<String, CityEntry>,
}

impl CitiesWeather {
    pub fn new() -> Self {
        CitiesWeather {
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, city: String, temp: f64) {
        // If the city is already in the map, update the entry
        if let Some(entry) = self.entries.get_mut(&city) {
            entry.update(temp);
        } else {
            // If the city is not in the map, create a new entry
            self.entries.insert(city, CityEntry::new(temp));
        }
    }

    //Our final getter
    pub fn into_sorted_vec(self) -> Vec<(String, CityEntry)> {
        let mut vec: Vec<(_, _)> = self.entries.into_iter().collect();
        vec.sort_by(|a, b| String::cmp(&a.0, &b.0));
        vec
    }
}

//Making fields private because updating needs logics

// It is good practice to implement common traits for custom types. :)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CityEntry {
    min_temp: f64,
    max_temp: f64,
    probes: u64,
    sum: f64,
}

impl CityEntry {
    // Creating a new entry, with only one probe
    pub fn new(temp: f64) -> Self {
        CityEntry {
            max_temp: temp,
            min_temp: temp,
            probes: 1,
            sum: temp,
        }
    }
    // Updating the data
    pub fn update(&mut self, temp: f64) {
        self.probes += 1;
        self.sum += temp;
        if temp > self.max_temp {
            self.max_temp = temp;
        }
        if temp < self.min_temp {
            self.min_temp = temp;
        }
    }

    //GETTERS

    pub fn avg(&self) -> f64 {
        self.sum / self.probes as f64
    }
    pub fn min(&self) -> f64 {
        self.min_temp
    }
    pub fn max(&self) -> f64 {
        self.max_temp
    }
}

impl Display for CityEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let round = |x: f64| -> f64 { (x * 10.0).round() / 10.0 };

        write!(
            f,
            "{}/{}/{}",
            round(self.min()),
            round(self.max()),
            round(self.avg())
        )
    }
}

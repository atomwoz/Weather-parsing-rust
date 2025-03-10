use feruca::Collator;
use hashbrown::HashMap;
use std::fmt::Display;

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
    pub fn into_sorted_ascii_vec(self) -> Vec<(String, CityEntry)> {
        let mut vec: Vec<(_, _)> = self.entries.into_iter().collect();
        //SORTING USING ORDINARY SORT
        vec.sort_by(|a, b| String::cmp(&a.0, &b.0));
        vec
    }
    pub fn into_slow_unicode_vec(self) -> Vec<(String, CityEntry)> {
        let mut vec: Vec<(_, _)> = self.entries.into_iter().collect();
        //SORTING USING FERUCA'S UNICODE ORDERING
        let mut collator = Collator::default();
        vec.sort_by(|a, b| collator.collate(&a.0, &b.0));
        vec
    }
}

//For usage convenience, implement From trait [witch is better than Into, because it automatically implements Into]
impl From<CitiesWeather> for Vec<(String, CityEntry)> {
    fn from(cities_weather: CitiesWeather) -> Self {
        cities_weather.into_slow_unicode_vec()
    }
}

// It is good practice to implement common traits for custom types. :)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
//Making fields private because updating needs logics
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
    pub fn to_string(&self) -> String {
        let round = |x: f64| -> f64 { (x * 10.0).round() / 10.0 };
        let mut s = String::new();
        s.push_str(round(self.min()).to_string().as_str());
        s.push('/');
        s.push_str(round(self.max()).to_string().as_str());
        s.push('/');
        s.push_str(round(self.avg()).to_string().as_str());
        s
    }
}

impl Display for CityEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let round = |x: f64| -> f64 { (x * 10.0).round() / 10.0 };

        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            round(self.min()),
            round(self.max()),
            round(self.avg())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_entry() {
        let mut entry = CityEntry::new(20.0);
        assert_eq!(entry.min(), 20.0);
        assert_eq!(entry.max(), 20.0);
        assert_eq!(entry.avg(), 20.0);

        entry.update(10.0);
        assert_eq!(entry.min(), 10.0);
        assert_eq!(entry.max(), 20.0);
        assert_eq!(entry.avg(), 15.0);

        entry.update(30.0);
        assert_eq!(entry.min(), 10.0);
        assert_eq!(entry.max(), 30.0);
        assert_eq!(entry.avg(), 20.0);
    }

    #[test]
    fn test_display_format() {
        let mut entry = CityEntry::new(20.5);
        entry.update(10.4);
        entry.update(30.6);
        assert_eq!(format!("{}", entry), "10.4/30.6/20.5");
    }

    #[test]
    fn test_cities_weather_with_unicode() {
        let mut cities = CitiesWeather::new();
        cities.add("Paris".to_string(), 20.0);
        cities.add("London".to_string(), 15.0);
        cities.add("Paris".to_string(), 25.0);
        cities.add("Óębłąk".to_string(), 25.0);

        let sorted = cities.into_slow_unicode_vec();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].0, "London");
        assert_eq!(sorted[1].0, "Óębłąk");
        assert_eq!(sorted[2].0, "Paris");
        assert_eq!(sorted[2].1.avg(), 22.5);
    }
    #[test]
    fn test_cities_weather_with_ascii() {
        let mut cities = CitiesWeather::new();
        cities.add("Paris".to_string(), 20.0);
        cities.add("London".to_string(), 15.0);
        cities.add("Paris".to_string(), 25.0);
        cities.add("Óębłąk".to_string(), 25.0);

        let sorted = cities.into_sorted_ascii_vec();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].0, "London");
        assert_eq!(sorted[1].0, "Paris");
        assert_eq!(sorted[2].0, "Óębłąk");
        assert_eq!(sorted[1].1.avg(), 22.5);
    }
}

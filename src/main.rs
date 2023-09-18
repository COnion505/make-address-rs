use std::{ env, error::Error, io::Write, fs::File };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    name_kanji: String,
    name_kana: String,
}

struct AddressHandler {
    states: Vec<Record>,
    cities: Vec<Record>,
    towns: Vec<Record>,
}
impl AddressHandler {
    fn new(states: Vec<Record>, cities: Vec<Record>, towns: Vec<Record>) -> AddressHandler {
        AddressHandler { states, cities, towns }
    }
    fn generate(&self, count: i32) -> Vec<Address> {
        use rand::prelude::*;
        let mut results = Vec::<Address>::new();
        let states_count = self.states.len();
        let cities_count = self.cities.len();
        let towns_count = self.towns.len();
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let states_index = rng.gen_range(0..states_count);
            let cities_index = rng.gen_range(0..cities_count);
            let towns_index = rng.gen_range(0..towns_count);
            let street_1 = rng.gen_range(0..5);
            let street_2 = rng.gen_range(0..10);
            let street_3 = rng.gen_range(0..20);

            results.push(Address {
                state: self.states.get(states_index).unwrap().clone(),
                city: self.cities.get(cities_index).unwrap().clone(),
                town: self.towns.get(towns_index).unwrap().clone(),
                street: format!("{}-{}-{}", street_1+1, street_2+1, street_3+1),
            })
        }
        results
    }
}
#[derive(Clone, Debug)]
struct Address {
    state: Record,
    city: Record,
    town: Record,
    street: String,
}
impl Address {
    fn full_kanji(&self) -> String {
        format!("{}{}{}{}", self.state.name_kanji, self.city.name_kanji, self.town.name_kanji, self.street)
    }
    fn full_kana(&self) -> String {
        format!("{}{}{}{}", self.state.name_kana, self.city.name_kana, self.town.name_kana, self.street)
    }
    fn full_csv(&self) -> String {
        format!(r#""{}","{}""#, &self.full_kanji(), &self.full_kana()) 
    }
}

fn import_csv(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    let mut cities: Vec<Record> = Vec::new();
    for result in rdr?.deserialize() {
        cities.push(result?);
    }
    Ok(cities)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: i32 = args.get(1).unwrap().parse().unwrap();
    let file_name = args.get(2);
    let cities = import_csv("csv/cities.csv").expect("import cities error");
    let states = import_csv("csv/states.csv").expect("import cities error");
    let towns = import_csv("csv/towns.csv").expect("import cities error");

    let address_handler = AddressHandler::new(states, cities, towns);
    let addresses = address_handler.generate(count);

    if let None = file_name {
        for address in addresses {
            println!("{}", address.full_csv());
        }
    } else {
        let mut fs = File::create("address.csv").expect("file create failed");
        for address in addresses {
            let str_address = address.full_csv();
            writeln!(fs,"{}",str_address).expect("output failed");
        }
    }
}

use clap::{arg, Command};
use std::collections::BTreeMap;
use tokio;
mod api;
use api::NearbyPlaces;
use geoutils::Location;

#[tokio::main]
async fn main() {
    let args = Command::new("poi")
        .arg(
            arg!(--lat <LATITUDE> "Latitude of the location")
                .required(false)
                .default_value("13.0827")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            arg!(--lng <LONGITUDE> "Longitude of the location")
                .required(false)
                .default_value("80.2707")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            arg!(-r --radius <RADIUS> "Radius from the location to search for, max 50000m")
                .required(false)
                .default_value("50000")
                .value_parser(clap::value_parser!(isize)),
        )
        .arg(
            arg!(-s --search <SEARCH> "Item to search for")
                .required(false)
                .default_value("restaurant"),
        )
        .arg(
            arg!(-c --category <DISTANCE_CATEGORY> "Distance in KM to categorize the results")
                .required(false)
                .default_value("2")
                .value_parser(clap::value_parser!(isize)),
        )
        .get_matches();
    let lat = *args
        .get_one::<f64>("lat")
        .expect("Cannot be None because of default val");
    let lng = *args
        .get_one::<f64>("lng")
        .expect("Cannot be None because of default val");
    let max_radius = *args
        .get_one::<isize>("radius")
        .expect("Cannot be None because of default val");
    let search_for = args
        .get_one::<String>("search")
        .expect("Cannot be None because of default val");
    let distance_category = *args
        .get_one::<isize>("category")
        .expect("Cannot be None because of default val");

    let nearby_places = NearbyPlaces::from((lat, lng), max_radius, search_for).await;
    print_sorted_by_distance(&nearby_places, distance_category);
}

// step_size in km
fn print_sorted_by_distance(nearby: &NearbyPlaces, step_size: isize) {
    let mut total_places: BTreeMap<isize, isize> = BTreeMap::new();
    let mut open_places: BTreeMap<isize, isize> = BTreeMap::new();

    let search_loc = Location::new(nearby.location.lat, nearby.location.lng);
    nearby
        .places
        .iter()
        .for_each(|(_place_id, (open_now, loc))| {
            let geo_loc = Location::new(loc.lat, loc.lng);
            if let Ok(distance) = search_loc.distance_to(&geo_loc) {
                let distance = distance.meters() / 1000.0;
                let distance_category = ceil_closest_to_the_multiple_of_step(distance, step_size);
                // increment counter if present, else initialize it with 1
                total_places
                    .entry(distance_category)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                if *open_now {
                    open_places
                        .entry(distance_category)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
        });

    // if discrepancy between the two counters, insert 0 into open places
    if total_places.len() != open_places.len() {
        total_places.keys().for_each(|key| {
            open_places.entry(*key).or_insert(0);
        })
    }

    println!("KM      No of places       Open now");
    total_places
        .into_iter()
        .zip(open_places.into_iter())
        .for_each(|((km, total), (_, open))| {
            println!(
                "{}-{km}          {total}               {open}",
                km - step_size
            );
        })
}

// given a num and step size, find the closest multiple of step to which it belongs to
fn ceil_closest_to_the_multiple_of_step(num: f64, step: isize) -> isize {
    let mut multiplier: isize = 1;
    loop {
        if num >= step as f64 * (multiplier - 1) as f64 && num < step as f64 * multiplier as f64 {
            return multiplier * step;
        }
        multiplier += 1;
    }
}

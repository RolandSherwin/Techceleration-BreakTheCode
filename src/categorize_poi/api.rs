use eyre::{eyre, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env};

type OpenNow = bool;
#[derive(Debug, Serialize, Deserialize)]
struct NearbyPlacesParser {
    #[serde(alias = "results")]
    places: Vec<Place>,
    next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Place {
    place_id: String,
    opening_hours: Option<OpeningHours>,
    geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpeningHours {
    open_now: OpenNow,
}

#[derive(Debug, Serialize, Deserialize)]
struct Geometry {
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Location {
    pub(crate) lat: f64,
    pub(crate) lng: f64,
}

#[derive(Debug)]
pub(crate) struct NearbyPlaces {
    pub(crate) location: Location,
    pub(crate) places: BTreeMap<String, (OpenNow, Location)>,
}

impl NearbyPlaces {
    pub async fn from(location: (f64, f64), max_radius: isize, search_for: &str) -> Self {
        let api_key = env::var("GOOGLE_DEV_API_KEY").expect("Set GOOGLE_DEV_API_KEY");
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={}%2C{}&radius={}&type={}&key={}",
            location.0, location.1, max_radius, search_for, api_key
        );
        let mut nearby_places = NearbyPlaces {
            location: Location {
                lat: location.0,
                lng: location.1,
            },
            places: BTreeMap::new(),
        };
        let mut done = false;
        while !done {
            // dont panic
            let parsed = Self::single_req(&url).await.expect("error fetching api");
            if parsed.places.is_empty() {
                // next_page_token is active only after sometime;
                // todo sleep instead of retrying
                continue;
            }
            println!("... fetching");
            parsed
                .places
                .into_iter()
                // .filter(|place| place.opening_hours.is_some())
                .for_each(|place| {
                    // if opening_hours is not present, consider the shop as closed;
                    let open_now = place
                        .opening_hours
                        .unwrap_or(OpeningHours { open_now: false });
                    nearby_places
                        .places
                        .insert(place.place_id, (open_now.open_now, place.geometry.location));
                });
            if let Some(next_page_token) = parsed.next_page_token {
                url = format!(
                    "https://maps.googleapis.com/maps/api/place/nearbysearch/json?pagetoken={}&key={}",
                next_page_token, api_key);
            } else {
                done = true
            }
        }
        nearby_places
    }

    async fn single_req(url: &String) -> Result<NearbyPlacesParser> {
        let response = reqwest::get(url).await?;
        let parsed = match response.status() {
            reqwest::StatusCode::OK => response.json::<NearbyPlacesParser>().await.unwrap(),
            reqwest::StatusCode::UNAUTHORIZED => panic!("Check your GOOGLE_DEV_API_KEY"),
            _ => return Err(eyre!("Error fetching response from server")),
        };
        Ok(parsed)
    }
}

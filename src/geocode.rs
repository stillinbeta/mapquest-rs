extern crate reqwest;
extern crate serde;

use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ReverseGeocodeResponse {
    /// This field contains information about the response.
    info: Info,
    options: Options,
    results: Vec<ReverseGeocodeResult>,
}

#[derive(Debug, Deserialize)]
pub struct GeocodeResponse {
    /// This field contains information about the response.
    info: Info,
    options: Options,
    results: Vec<GeocodeResult>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    /// HTTP status codes
    /// See https://developer.mapquest.com/documentation/geocoding-api/status-codes/
    status_code: u32,
    /// The messages subfield is an array of error messages that describe the status
    messages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Options {
    #[serde(rename = "maxResults")]
    pub max_results: u32,
    #[serde(rename = "thumbMaps")]
    pub thumb_maps: bool,
    #[serde(rename = "ignoreLatLngInput")]
    pub ignore_lat_lng_input: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LatLng {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Deserialize)]
pub struct ReverseGeocodeResult {
    #[serde(rename = "providedLocation")]
    /// The provided location properties passed in the geocode request.
    pub provided_location: LatLng,
    ///
    pub locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
pub struct GeocodeResult {
    #[serde(rename = "providedLocation")]
    pub provided_location: ProvidedLocation,
    pub locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
pub struct ProvidedLocation {
    location: String,
}

#[derive(Debug)]
pub enum LocationType {
    Stop,
    Via,
}

impl<'de> Deserialize<'de> for LocationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "s" => Ok(LocationType::Stop),
            "v" => Ok(LocationType::Via),
            v => Err(D::Error::unknown_variant(&v, &["s", "v"])),
        }
    }
}

#[derive(Debug)]
pub enum SideOfStreet {
    Left,
    Right,
    Mixed,
    None,
}

impl<'de> Deserialize<'de> for SideOfStreet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "r" => Ok(SideOfStreet::Right),
            "l" => Ok(SideOfStreet::Left),
            "m" => Ok(SideOfStreet::Mixed),
            "n" => Ok(SideOfStreet::None),
            v => Err(D::Error::unknown_variant(&v, &["r", "l", "m", "n"])),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Location {
    /// Street address
    pub street: String,
    /// Neighborhood Name
    #[serde(rename = "adminArea6")]
    pub admin_area_6: String,
    /// City name
    #[serde(rename = "adminArea5")]
    pub admin_area_5: String,
    /// County name
    #[serde(rename = "adminArea4")]
    pub admin_area_4: String,
    /// State name
    #[serde(rename = "adminArea3")]
    pub admin_area_3: String,
    /// Country name
    #[serde(rename = "adminArea4")]
    pub admin_area_1: String,

    /// Type of location.
    #[serde(rename = "type")]
    pub location_type: LocationType,

    /// Is location a drag point? This option only applies when making a `dragroute` call.
    #[serde(rename = "dragPoint")]
    pub drag_point: bool,

    /// A lat/lng pair that can be helpful when showing this address as a Point of Interest.
    #[serde(rename = "displayLatLng")]
    pub display_lat_lng: LatLng,

    /// Specifies the side of street.
    #[serde(rename = "sideOfStreet")]
    pub side_of_street: SideOfStreet,

    /// https://developer.mapquest.com/documentation/geocoding-api/quality-codes/
    #[serde(rename = "geocodeQualityCode")]
    pub geocode_quality_code: String,
    /// https://developer.mapquest.com/documentation/geocoding-api/quality-codes/
    #[serde(rename = "geocodeQuality")]
    pub geocode_quality: String,

    /// String that identifies the closest road to the address for routing purposes.
    #[serde(rename = "linkId")]
    pub link_id: String,
}

pub struct Client<'a> {
    client: reqwest::Client,
    api_key: &'a str,
}

impl<'a> Client<'a> {
    const BASE_URL: &'static str = "http://www.mapquestapi.com/geocoding/v1";
    /// Create a new Mapquest client.
    /// You can get an API key at https://developer.mapquest.com/user/me/apps
    pub fn new(api_key: &'a str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }
    /// Reverse geocoding is the process of taking a latitude and longitude pair and providing the associated address, or nearest address point.
    /// See https://developer.mapquest.com/documentation/geocoding-api/reverse/get
    pub fn reverse_geocode(
        &self,
        lat: f32,
        lng: f32,
    ) -> Result<ReverseGeocodeResponse, crate::Error> {
        Ok(self
            .client
            .get(&format!("{}/reverse", Self::BASE_URL))
            .query(&[
                ("key", self.api_key),
                ("location", &format!("{:},{}", lat, lng)),
            ])
            .send()?
            .json()?)
    }

    /// Forward geocoding (also called address geocoding) is the process of finding an associated latitude and longitude for a given address.
    /// See https://developer.mapquest.com/documentation/geocoding-api/address/get/
    pub fn geocode(&self, address: &str) -> Result<ReverseGeocodeResponse, crate::Error> {
        Ok(self
            .client
            .get(&format!("{}/address", Self::BASE_URL))
            .query(&[("key", self.api_key), ("location", address)])
            .send()?
            .json()?)
    }
}

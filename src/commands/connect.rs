use std::collections::HashMap;
use keride::cesr::common::Tierage;
use keride::cesr::Sadder;
use crate::app::authing::Controller;

pub(crate) fn connect(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url).unwrap();
    println!("{:#?}", resp.headers());
    println!("{:#?}", resp.json::<HashMap<String, String>>()?);
    let bran = "0123456789abcdefghijk";
    let controller = Controller::new(Some(bran), Some(&Tierage::low.to_string()), Some("")).unwrap();

    println!("{}", controller.serder.pretty(None).unwrap());
    Ok(())
}
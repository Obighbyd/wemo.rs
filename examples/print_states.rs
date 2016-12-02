// Copyright (c) 2015 Brandon Thomas <bt@brand.io>
extern crate time;
extern crate wemo;

use time::Duration;
use wemo::DeviceSearch;
use wemo::Switch;

pub fn main() {
  let mut search = DeviceSearch::new();
  let results = search.search(5_000);

  for device in results.values() {
    let device = Switch::from_dynamic_ip_and_port(device.ip_address,
        device.port);

    match device.get_state_with_retry(Duration::seconds(3)) {
      Err(_) => { println!("Could not get the state."); },
      Ok(state) => {
        println!("Device {} turned on: {}", device.location(), state.is_on());
      },
    }
  }
}


use serde_json::json;

mod position;
mod wireless;

use crate::position::*;
use crate::wireless::*;

const LIGHTSPEED: f64 = 299792458.0;

fn main() {
    // init the beacons!
    let beac1 = Beacon {
        x: 1.0,
        y: 1.0,
        time: 1,
    };
    let beac2 = Beacon {
        x: 10.0,
        y: 10.0,
        time: 1,
    };
    let beac3 = Beacon {
        x: 7.0,
        y: 4.0,
        time: 1,
    };

    // this reciever doesn't know where they are
    let _recp = Reciever {
        x: 3.0,
        y: 3.0,
        time: 1,
    };

    let time_from_beacon_1_to_reciver = 0.000000026685128;
    let time_from_beacon_2_to_reciver = 0.000000326892813;
    let time_from_beacon_3_to_reciver = 0.000000056705896;

    println!(
        "{}\n{}\n{}",
        (time_from_beacon_1_to_reciver * LIGHTSPEED).sqrt(),
        (time_from_beacon_2_to_reciver * LIGHTSPEED).sqrt(),
        (time_from_beacon_3_to_reciver * LIGHTSPEED).sqrt(),
    );

    let results = trilaterate_from_distance(
        beac1,
        beac2,
        beac3,
        (time_from_beacon_1_to_reciver * LIGHTSPEED).sqrt(),
        (time_from_beacon_2_to_reciver * LIGHTSPEED).sqrt(),
        (time_from_beacon_3_to_reciver * LIGHTSPEED).sqrt(),
    );

    let _json = json!(results);

    println!("{}", serde_json::to_string_pretty(&_json).unwrap());

    //// IF WE DO WIFI
    let f = 2442.0; //mhz (7'th 802.11bgn channel),

    let ptx = 16.0; //dbm,
    let cltx = 0.0; //,
    let agtx = 2.0; //dbi,
    let agrx = 0.0; //,
    let clrx = 0.0;
    // let prx = -60.0; //dbm (received signal strength),
    let fm = 22.0;

    let dist1 = wifi_strength_to_meters(f, ptx, cltx, agtx, agrx, clrx, -53.235, fm);
    println!("{:?}", dist1);
    let dist2 = wifi_strength_to_meters(f, ptx, cltx, agtx, agrx, clrx, -64.117, fm);
    println!("{:?}", dist2);
    let dist3 = wifi_strength_to_meters(f, ptx, cltx, agtx, agrx, clrx, -56.511, fm);
    println!("{:?}", dist3);

    let results = trilaterate_from_distance(beac1, beac2, beac3, dist1, dist2, dist3);
    // let results = trilaterate_from_distance(beac1, beac2, beac3, 8.0, 98.0, 17.0);

    let _json = json!(results);

    println!("{}", serde_json::to_string_pretty(&_json).unwrap());
    // ()
    // println!("_counter {}", _counter);
}

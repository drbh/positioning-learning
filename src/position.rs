use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub struct Beacon {
    pub x: f64,
    pub y: f64,
    pub time: i64,
}

#[derive(Debug)]
pub struct Reciever {
    pub x: f64,
    pub y: f64,
    pub time: i64,
}

impl Beacon {
    pub fn parametric(self: Self, r: f64, a: f64) -> (f64, f64) {
        let x = self.x + r * a.cos();
        let y = self.y + r * a.sin();
        (x, y)
    }
}

pub fn trilaterate_from_distance(
    beac1: Beacon,
    beac2: Beacon,
    beac3: Beacon,

    distance1: f64,
    distance2: f64,
    distance3: f64,
) -> HashMap<&'static str, (f64, f64)> {
    // number of loops around the circle
    let n = 1000;
    let k = (2.0 * std::f64::consts::PI) / n as f64;
    let error_margin = 0.001;
    let error_margin_two = 0.03;

    // setup a _counter and bool for early 'break'
    let mut _counter = 0;
    let mut didfind = false;
    let mut results = HashMap::new();

    let now = SystemTime::now();
    // loop over the circle segments
    for i in 1..n {
        // get the x, y of the circle at that point
        let rad = k * i as f64;
        let v = distance1;
        let b1 = beac1.parametric(v, rad);

        // now we are going to look for any matches with the second circle
        for i in 1..n {
            let rad = k * i as f64;
            let v = distance2;
            let b2 = beac2.parametric(v, rad);
            if (b1.0 - b2.0).abs() < error_margin {
                if (b1.1 - b2.1).abs() < error_margin {
                    results.insert("A", b1);
                    results.insert("B", b2);
                    didfind = true;

                    break;
                }
            }

            _counter += 1;
        }
        if didfind {
            break;
        }
    }

    for l in 1..n {
        let rad = k * l as f64;
        let v = distance3;
        let b3 = beac3.parametric(v, rad);
        let b2 = results.get("A").unwrap();
        if (b3.0 - b2.0).abs() < error_margin {
            // println!("\n{} {} {:?} {}", l, v, b2, (b3.0 - b2.0).abs());
            // println!("{} {} {:?} {}", l, v, b2, (b3.1 - b2.1).abs());
            if (b3.1 - b2.1).abs() < error_margin_two {
                results.insert("C", b3);
                // println!("C {:?}", b3);
                break;
            }
        }
        _counter += 1;
    }

    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("elapsed nano seconds: {}", elapsed.as_nanos());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }

    results
}

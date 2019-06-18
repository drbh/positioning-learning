use serde_json::json;

#[derive(Debug, Clone, Copy)]
struct Beacon {
    x: f64,
    y: f64,
    time: i64,
}

#[derive(Debug)]
struct Reciever {
    x: f64,
    y: f64,
    time: i64,
}

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
    let recp = Reciever {
        x: 3.0,
        y: 3.0,
        time: 1,
    };

    // number of loops around the circle
    let n = 1000;
    let k = (2.0 * std::f64::consts::PI) / n as f64;
    let error_margin = 0.001;
    let error_margin_two = 0.03;

    // setup a counter and bool for early 'break'
    let mut counter = 0;
    let mut didfind = false;

    let mut results = std::collections::HashMap::new();

    // loop over the circle segments
    for i in 1..n {
        // get the x, y of the circle at that point
        let rad = k * i as f64;
        let v = 8.0 as f64;
        let b1 = beac1.parametric(v.sqrt(), rad);

        // now we are going to look for any matches with the second circle
        for i in 1..n {
            let rad = k * i as f64;
            let v = 98.0 as f64;
            let b2 = beac2.parametric(v.sqrt(), rad);
            if (b1.0 - b2.0).abs() < error_margin {
                if (b1.1 - b2.1).abs() < error_margin {
                    results.insert("A", b1);
                    results.insert("B", b2);
                    didfind = true;

                    break;
                }
            }

            counter += 1;
        }
        if didfind {
            break;
        }
    }

    for l in 1..n {
        let rad = k * l as f64;
        let v = 17.0 as f64;
        let b3 = beac3.parametric(v.sqrt(), rad);
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
        counter += 1;
    }

    let _json = json!(results);
    println!("Result");

    println!("{}", serde_json::to_string_pretty(&_json).unwrap());

    // println!("counter {}", counter);
}

impl Beacon {
    fn parametric(self: Self, r: f64, a: f64) -> (f64, f64) {
        let x = self.x + r * a.cos();
        let y = self.y + r * a.sin();
        (x, y)
    }
}

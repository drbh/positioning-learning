pub fn wifi_strength_to_meters(
    f: f64,
    ptx: f64,
    cltx: f64,
    agtx: f64,
    agrx: f64,
    clrx: f64,
    prx: f64,
    fm: f64,
) -> f64 {
    // based on https://stackoverflow.com/a/27238116
    // let k = 32.44;
    let k = -27.55;
    let fspl = ptx - cltx + agtx + agrx - clrx - prx - fm;
    let d = (10.0 as f64).powf((fspl - k - 20.0 * f.log10()) / 20.0);
    d
}

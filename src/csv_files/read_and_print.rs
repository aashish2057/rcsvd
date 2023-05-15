use csv;
pub fn read(path: &str) {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    println!("{:?}", rdr);
}

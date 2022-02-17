use std::fs::File;
use crate::risks::risks::{RiskMap, Coord};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::collections::HashMap;
use std::borrow::Borrow;
use crate::bicriteria_dijkstra::bicriteria_dijkstra::{BicriteriaDijkstraInstance, Path};
use std::time::Instant;
use crate::air_risks::air_risks::AirRiskInstance;
use serde::{Serialize, Deserialize};
use std::fs;

mod risks;
mod air_risks;
mod bicriteria_dijkstra;

fn main() {
    let mut colors: HashMap<&[u8; 4], i32> = HashMap::new();
    colors.insert(&[255, 255, 255, 255], 1);
    colors.insert(&[214, 214, 214, 255], 4);
    colors.insert(&[180, 209, 82, 255], 19);
    colors.insert(&[183, 103, 26, 255], 199);
    colors.insert(&[109, 0, 65, 255], 499);
    colors.insert(&[27, 0, 31, 255], 1000);


    let total_time = 4*7*24;
    let map = load_map_from_image("./data/density_fixed_scaled.png", &colors);
    let air_risk_instance = load_air_risk_map("./data/map.json", total_time);

    assert_eq!(map[0].len(), air_risk_instance.map.len());
    assert_eq!(map.len(), air_risk_instance.map[0].len());

    let risk_map = RiskMap{
        map: map,
        m_per_pixel: 1000.0/(131.0/2.0),
        offset: 25
    };

    // let inst = BicriteriaDijkstraInstance::new(&risk_map, Coord{x: 500, y: 500}, Coord{x: 600, y: 600}, 5, 150.0);
    let inst = BicriteriaDijkstraInstance::new(&risk_map, Coord{x: 517, y: 412}, Coord{x: 765, y: 600}, 5, 150.0);

    let start = Instant::now();

    let paths = inst.compute_pareto_apx_paths();

    println!("{:?}", paths);

    let mut res_routes = vec![];

    for path in paths {
        let air_risk = air_risk_instance.compute_air_risk(&path);
        res_routes.push(HFRMPath{
            route: path.path,
            air_risk: air_risk,
            ground_risk: path.risk as f64,
            length_m: path.length_m,
            alpha: path.alpha
        })
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);

    save_paths_to_json("./results/res_nk.json", &res_routes);
}

fn load_map_from_image(image: &str, colors: &HashMap<&[u8; 4], i32>) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = vec![];
    let img = ImageReader::open(image).unwrap().decode().unwrap();

    for y in 0..img.height() {
        let mut line: Vec<i32> = vec![];

        for x in 0..img.width() {
            let px = img.get_pixel(x, y);
            let risk: &i32 = colors.get(&px.0).unwrap();

            line.push(*risk);
        }

        map.push(line);
    }

    return map
}

fn load_air_risk_map(map_filename: &str, total_time_s: i32) -> AirRiskInstance {
    let mut air_risk_map = serde_json::from_reader(File::open(map_filename).unwrap()).unwrap();

    return AirRiskInstance::new(air_risk_map, total_time_s);
}

fn save_paths_to_json(filename: &str, paths: &Vec<HFRMPath>) {
    let j = serde_json::to_string(paths).unwrap();
    fs::write(filename, j).expect("Unable to write file");
    // println!("{}", &j)
}

#[derive(Serialize)]
struct HFRMPath {
    route: Vec<Coord<i16>>,
    air_risk: f64,
    ground_risk: f64,
    length_m: f64,
    alpha: f64,
}
use crate::bicriteria_dijkstra::bicriteria_dijkstra::Path;
use bresenham::Bresenham;

pub struct AirRiskInstance {
    pub(crate) map: Vec<Vec<i32>>,
    total_time_s: i32
}

impl AirRiskInstance {
    pub fn new(map: Vec<Vec<i32>>, total_time_s: i32) -> Self {
        return Self{ map, total_time_s}
    }

    pub fn compute_air_risk(&self, path: &Path) -> f64 {
        let mut air_risk = 0.0;
        let mut length_px = 0;

        for i in 0..&path.path.len()-1 {
            let s = &path.path[i];
            let e = &path.path[i+1];

            for (x, y) in Bresenham::new((s.x as isize, s.y as isize), (e.x as isize, e.y as isize)) {
                air_risk += self.map[x as usize][y as usize] as f64;
                length_px += 1;
            }
        }

        return (air_risk/(length_px as f64))/(self.total_time_s as f64)
    }
}
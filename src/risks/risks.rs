#[cfg(test)]
#[path = "tests.rs"]
mod tests;
use itertools::Itertools;
use core::cmp;
use ordered_float::OrderedFloat;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

pub struct RiskMap {
    pub map: Vec<Vec<i32>>,
    pub m_per_pixel: f64,
    pub offset: i16
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, Serialize)]
pub struct Coord <T> {
    pub x: T,
    pub y: T
}

impl Coord<f64> {
    fn round_if_needed(&mut self) {
        // A dirty hack to avoid numerical problems
        //FIXME: do something more reliable
        if (self.x.round() - self.x).abs() < 0.0001 {
            self.x = self.x.round();
        }

        if (self.y.round() - self.y).abs() < 0.0001 {
            self.y = self.y.round();
        }
    }
}

impl RiskMap {
    pub fn height(&self) -> i16 {
        let height = self.map.len() as i16 - self.offset*2;
        if height < 1 {
            panic!("The map is smaller than the offset");
        }

        return height as i16
    }

    pub fn width(&self) -> i16 {
        let width = self.map.get(0).unwrap().len() as i16 - self.offset*2;
        if width < 1 {
            panic!("The map is smaller than the offset");
        }
        return width as i16
    }

    fn parallelogram_risk(&self, origin_side: (Coord<f64>, Coord<f64>), destination_side: (Coord<f64>, Coord<f64>)) -> i32 {
        let mut pop = 0;

        let rect = ParallelogramPixels::new(origin_side, destination_side);

        for coord in rect.iter() {
            if 0 <= coord.x && coord.x < self.width() && 0 <= coord.y && coord.y < self.height() {
                pop = pop + self.risk_at(coord);
            }
        }

        return pop
    }

    pub fn risk_at(&self, coord: Coord<i16>) -> i32 {
        return self.map[coord.y as usize][coord.x as usize];
    }

    fn parallelogram_from_two_points(p1: Coord<i16>, p2: Coord<i16>, r_m: f64, m_per_pixel: f64) -> ((Coord<f64>, Coord<f64>), (Coord<f64>, Coord<f64>)) {
        let y_diff = -(p2.y-p1.y) as f64;
        let x_diff = (p2.x-p1.x) as f64;
        let slope = y_diff.atan2(x_diff);
        let rect_width = (r_m /m_per_pixel).ceil();

        let orig_p1 = Coord{
            x: p1.x as f64 + slope.sin() * rect_width,
            y: p1.y as f64 + slope.cos() * rect_width
        };

        let orig_p2 = Coord{
            x: p1.x as f64 + (slope + std::f64::consts::PI).sin() * rect_width,
            y: p1.y as f64 + (slope + std::f64::consts::PI).cos() * rect_width
        };

        let dest_p1 = Coord{
            x: p2.x as f64 + slope.sin() * rect_width,
            y: p2.y as f64 + slope.cos() * rect_width
        };

        let dest_p2 = Coord{
            x: p2.x as f64 + (slope + std::f64::consts::PI).sin() * rect_width,
            y: p2.y as f64 + (slope + std::f64::consts::PI).cos() * rect_width
        };

        return ((orig_p1, orig_p2), (dest_p1, dest_p2))
    }

    pub fn risk(&self, p1: Coord<i16>, p2: Coord<i16>, r_m: f64) -> i32 {
        let (orig_side, dest_side) = Self::parallelogram_from_two_points(p1, p2, r_m, self.m_per_pixel);

        return self.parallelogram_risk(orig_side, dest_side);
    }

    pub fn neighbours_within(&self, p: Coord<i16>, search_limit: i16) -> NeighboursIter {
        return NeighboursIter::new(self, search_limit, p);
    }

    pub fn all_points_iterator(&self) -> AllPointsIter {
        return AllPointsIter::new(self);
    }

    pub fn length_m(&self, p1: Coord<i16>, p2: Coord<i16>) -> f64 {
        let x_diff = (p1.x - p2.x) as f64;
        let y_diff = (p1.y - p2.y) as f64;
        let length_px_squared = y_diff.powi(2) + x_diff.powi(2);
        let length_px = length_px_squared.sqrt();
        return length_px * self.m_per_pixel + 0.0000001;
    }
}

struct ParallelogramPixels {
    origin_side: (Coord<f64>, Coord<f64>),
    destination_side: (Coord<f64>, Coord<f64>)
}

impl ParallelogramPixels {
    fn new(origin_side: (Coord<f64>, Coord<f64>), destination_side: (Coord<f64>, Coord<f64>)) -> Self {
        let mut res = Self{ origin_side, destination_side };
        res.origin_side.0.round_if_needed();
        res.origin_side.1.round_if_needed();
        res.destination_side.0.round_if_needed();
        res.destination_side.1.round_if_needed();

        return res
    }
    fn iter(&self) -> ParallelogramPixelsIter {
        return ParallelogramPixelsIter::new(self)
    }
}

impl Display for ParallelogramPixels {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "original side: {} {}, destination side: {} {}", self.origin_side.0, self.origin_side.1, self.destination_side.0, self.destination_side.1)
    }
}

struct ParallelogramPixelsIter <'a> {
    parallelogram: &'a ParallelogramPixels,
    x: i16,
    y: i16,
    current_range: (i16, i16),
    l: &'a Coord<f64>,
    r: &'a Coord<f64>,
    b: &'a Coord<f64>,
    t: &'a Coord<f64>
}

pub struct NeighboursIter <'a> {
    map: &'a RiskMap,
    search_limit: i16,
    p: Coord<i16>,
    current_x: i16,
    current_y: i16,
    x_from: i16,
    x_to: i16,
    y_from: i16,
    y_to: i16
}

impl <'a> NeighboursIter <'a> {
    fn new(map: &'a RiskMap, search_limit: i16, p: Coord<i16>) -> Self {
        let width= map.width() as i16;
        let height = map.height() as i16;

        let x_from = cmp::max(0, p.x - map.offset - search_limit);
        let x_to = cmp::min(width - 1, p.x - map.offset + search_limit);
        let y_from = cmp::max(0, p.y - map.offset - search_limit);
        let y_to = cmp::min(height - 1, p.y - map.offset + search_limit);

        return NeighboursIter{
            map: map,
            search_limit,
            p,
            current_x: x_from,
            current_y: y_from,
            x_from: x_from,
            x_to: x_to,
            y_from: y_from,
            y_to: y_to
        }
    }

    fn propagate(&mut self) {
        if self.current_x >= self.x_to {
            self.current_x = self.x_from;
            self.current_y += 1;
        } else {
            self.current_x += 1;
        }
    }
}

impl <'a> Iterator for NeighboursIter <'a> {
    type Item = Coord<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x + self.map.offset == self.p.x && self.current_y + self.map.offset == self.p.y {
            self.propagate()
        }

        if self.current_y > self.y_to {
            return None
        }

        let res = Coord{x: self.current_x + self.map.offset, y: self.current_y + self.map.offset};

        self.propagate();

        return Some(res);
    }
}

pub struct AllPointsIter <'a> {
    map: &'a RiskMap,
    current_x: i16,
    current_y: i16
}

impl <'a> AllPointsIter <'a> {
    fn new(map: &'a RiskMap) -> Self {
        return Self{
            map,
            current_x: 0,
            current_y: 0
        }
    }
}

impl <'a> Iterator for AllPointsIter <'a> {
    type Item = Coord<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.map.height(){
            return None
        }

        let res = Coord{x: self.current_x + self.map.offset, y: self.current_y + self.map.offset};

        if self.current_x + 1 >= self.map.width() {
            self.current_y += 1;
            self.current_x = 0;
        } else {
            self.current_x += 1;
        }

        return Some(res);
    }
}

impl<'a> ParallelogramPixelsIter <'a> {
    fn new(par: &'a ParallelogramPixels) -> Self {
        let points: Vec<&Coord<f64>> = vec![&par.origin_side.0, &par.origin_side.1, &par.destination_side.0, &par.destination_side.1];
        let (l, mid_1, mid_2, r): (&Coord<f64>, &Coord<f64>, &Coord<f64>, &Coord<f64>) = points.iter()
            .cloned()
            .sorted_by_key(
                |k| (OrderedFloat(k.x), OrderedFloat(k.y))
            )
            .next_tuple()
            .unwrap();
        let mid_points: Vec<&Coord<f64>> = vec![mid_1, mid_2];
        let (b, t): (&Coord<f64>, &Coord<f64>) = mid_points.iter().cloned()
            .sorted_by_key(|k| (OrderedFloat(k.y), OrderedFloat(k.x)))
            .next_tuple().unwrap();
        let mut res = Self{parallelogram: par, x: l.x.floor() as i16, y: 0, current_range: (0, 0), l: l, r: r, b: b, t: t};
        let current_range = res.y_range(res.x);
        res.current_range = current_range;

        res.y = res.current_range.0;

        return res
    }

    fn y_range(&self, x: i16) -> (i16, i16) {
        let x = f64::from(x);

        if (self.l.x != self.b.x && (self.l.x - self.b.x).abs() < 0.001) || (self.t.y != self.r.y && (self.t.y - self.r.y).abs() < 0.001) {
            panic!("Numerical errors detected!");
        }

        let mut y1: i16 = 0;
        let mut y2: i16 = 0;

        if self.t.x < self.b.x {
            //   -------      ----
            //  /     /   or   \  \
            // ------           \  \
            //                   \  \
            //                    ----

            if x < self.t.x {
                y1 = get_min_y(&self.l, &self.b, x);
                y2 = get_max_y(&self.l, &self.t, x);
            } else if x >= self.t.x && x <= self.b.x {
                y1 = get_min_y(&self.l, &self.b, x);
                y2 = get_max_y(&self.t, &self.r, x);
            } else {
                y1 = get_min_y(&self.b, &self.r, x);
                y2 = get_max_y(&self.t, &self.r, x);
            }
        } else if self.t.x > self.b.x && self.t.y != self.b.y {
            //          -------           ----
            //           \     \   or    /   /
            //            ------        /   /
            //                         /   /
            //                         ----

            if x < self.b.x {
                y1 = get_min_y(&self.l, &self.b, x);
                y2 = get_max_y(&self.l, &self.t, x);
            } else if x >= self.b.x && x <= self.t.x {
                y1 = get_min_y(&self.b, &self.r, x);
                y2 = get_max_y(&self.l, &self.t, x);
            } else {
                y1 = get_min_y(&self.b, &self.r, x);
                y2 = get_max_y(&self.t, &self.r, x);
            }
        } else if self.t.x == self.b.x {
            //          /\
            //          \/
            if x < self.t.x {
                y1 = get_min_y(&self.l, &self.b, x);
                y2 = get_max_y(&self.l, &self.t, x);
            } else {
                y1 = get_min_y(&self.b, &self.r, x);
                y2 = get_max_y(&self.t, &self.r, x);
            }
        } else if self.t.x > self.b.x && self.b.y == self.t.y {
            y1 = get_min_y(&self.l, &self.t, x);
            y2 = get_max_y(&self.b, &self.r, x);
        } else {
            panic!("An unknown case occured!");
        }

        return (y1, y2)
    }
}

impl <'a> Iterator for ParallelogramPixelsIter<'a> {
    type Item = Coord<i16>;

    fn next(&mut self) -> Option<Coord<i16>> {
        if self.x <= self.r.x as i16 {
            let res = Some(Coord{x: self.x, y: self.y});

            if self.y > 5000 {
                println!("y is > 5000 = {}, {}, {}, {}", self.y, self.current_range.0, self.current_range.1, self.parallelogram);
            }

            if self.y + 1 > self.current_range.1 {
                self.x += 1;
                self.current_range = self.y_range(self.x);
                self.y = self.current_range.0;
            } else {
                self.y += 1;
            }

            return res
        } else {
            return None
        }
    }
}

fn get_min_y(p0: &Coord<f64>, p1: &Coord<f64>, x: f64) -> i16 {
    // Thanks to Markus Jarderot from https://stackoverflow.com/questions/5610616/finding-all-pixels-at-least-partially-within-an-arbitrarily-oriented-rectangle
    // for inspiration.

    let x0 = p0.x;
    let y0 = p0.y;
    let x1 = p1.x;
    let y1 = p1.y;

    if (x0 - x1).abs() < 0.00001 {
        return y0.floor() as i16;
    }

    let slope = (y1 - y0)/(x1 - x0);

    if slope >= 0.0 {
        let xl = cmp::max(OrderedFloat(x0), OrderedFloat(x - 0.5)).0;
        let res = y0 + slope * (xl - x0) + 0.00001;
        return res.round() as i16;
    } else {
        let xr = cmp::min(OrderedFloat(x1), OrderedFloat(x + 0.5)).0;
        let res = y0 + slope * (xr - x0) + 0.00001;
        return res.round() as i16;
    }
}

fn get_max_y(p0: &Coord<f64>, p1: &Coord<f64>, x: f64) -> i16 {
    // Thanks to Markus Jarderot from https://stackoverflow.com/questions/5610616/finding-all-pixels-at-least-partially-within-an-arbitrarily-oriented-rectangle
    // for inspiration.

    let x0 = p0.x;
    let y0 = p0.y;
    let x1 = p1.x;
    let y1 = p1.y;

    if (x0 - x1).abs() < 0.00001 {
        return y1.ceil() as i16;
    }

    let slope = (y1 - y0)/(x1 - x0);

    if slope >= 0.0 {
        let xr = cmp::min(OrderedFloat(x1), OrderedFloat(x + 0.5)).0;
        let res = y0 + slope * (xr - x0) - 0.00001;
        return res.round() as i16;
    } else {
        let xl = cmp::max(OrderedFloat(x0), OrderedFloat(x - 0.5)).0;
        let res = y0 + slope * (xl - x0) - 0.00001;
        return res.round() as i16;
    }
}


impl <T: Display> Display for Coord<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
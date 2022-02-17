#[cfg(test)]
mod tests {
    use super::super::{*};

    #[test]
    fn test_simple_rectangle() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 0.0, y: 3.0}),
            destination_side: (Coord {x: 2.0, y: 0.0}, Coord {x: 2.0, y: 3.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 12);
        assert_eq!(pixels, vec![Coord{x: 0, y:0},
                                Coord{x: 0, y: 1},
                                Coord{x: 0, y: 2},
                                Coord{x: 0, y: 3},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 1, y: 3},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 2, y: 3}]);
    }

    #[test]
    fn test_left_to_right_parallelogram() {

        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 1.0, y: 1.0}),
            destination_side: (Coord {x: 2.0, y: 0.0}, Coord {x: 3.0, y: 1.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 6);
        assert_eq!(pixels, vec![Coord{x: 0, y: 0},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 3, y: 1}]);


    }

    #[test]
    fn test_rhombus() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 2.0, y: 1.0}),
            destination_side: (Coord {x: 2.0, y: 0.0}, Coord {x: 4.0, y: 1.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 8);
        assert_eq!(pixels, vec![Coord{x: 0, y: 0},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 3, y: 0},
                                Coord{x: 3, y: 1},
                                Coord{x: 4, y: 1},
        ]);
    }

    #[test]
    fn test_rhombus_the_other_way() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 1.0}, Coord {x: 1.0, y: 0.0}),
            destination_side: (Coord {x: 1.0, y: 1.0}, Coord {x: 2.0, y: 0.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 4);
        assert_eq!(pixels, vec![Coord{x: 0, y: 1},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 2, y: 0},
        ]);
    }

    #[test]
    fn test_right_to_left_parallelogram() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 2.0}, Coord {x: 1.0, y: 0.0}),
            destination_side: (Coord {x: 3.0, y: 0.0}, Coord {x: 2.0, y: 2.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 10);
        assert_eq!(pixels, vec![Coord{x: 0, y: 1},
                                Coord{x: 0, y: 2},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 3, y: 0},
                                Coord{x: 3, y: 1},
        ]);
    }

    #[test]
    fn test_vertical_sides() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 0.0, y: 2.0}),
            destination_side: (Coord {x: 1.0, y: 2.0}, Coord {x: 1.0, y: 4.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 8);
        assert_eq!(pixels, vec![Coord{x: 0, y: 0},
                                Coord{x: 0, y: 1},
                                Coord{x: 0, y: 2},
                                Coord{x: 0, y: 3},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 1, y: 3},
                                Coord{x: 1, y: 4},
        ]);
    }

    #[test]
    fn test_left_to_right_narrow() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 2.0}, Coord {x: 1.0, y: 2.0}),
            destination_side: (Coord {x: 2.0, y: 0.0}, Coord {x: 3.0, y: 0.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 6);
        assert_eq!(pixels, vec![Coord{x: 0, y: 2},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 3, y: 0},
        ]);
    }

    #[test]
    fn test_left_to_right_narrow_2() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 3.0}, Coord {x: 1.0, y: 3.0}),
            destination_side: (Coord {x: 2.0, y: 0.0}, Coord {x: 3.0, y: 0.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 10);
        assert_eq!(pixels, vec![Coord{x: 0, y: 2},
                                Coord{x: 0, y: 3},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 1, y: 3},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 3, y: 0},
                                Coord{x: 3, y: 1},
        ]);
    }

    #[test]
    fn test_right_to_left() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 1.0, y: 0.0}),
            destination_side: (Coord {x: 2.0, y: 3.0}, Coord {x: 3.0, y: 3.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 10);
        assert_eq!(pixels, vec![Coord{x: 0, y: 0},
                                Coord{x: 0, y: 1},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 2, y: 3},
                                Coord{x: 3, y: 2},
                                Coord{x: 3, y: 3},
        ]);
    }

    #[test]
    fn test_right_to_left_2() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 1.0, y: 0.0}),
            destination_side: (Coord {x: 2.0, y: 2.0}, Coord {x: 3.0, y: 2.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 6);
        assert_eq!(pixels, vec![Coord{x: 0, y: 0},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 3, y: 2},
        ]);
    }

    #[test]
    fn test_rectangles() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 0.0, y: 0.0}, Coord {x: 1.0, y: 2.0}),
            destination_side: (Coord {x: 5.0, y: 0.0}, Coord {x: 4.0, y: -2.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 20);
        assert_eq!(pixels, vec![Coord{x: 0, y: 0},
                                Coord{x: 0, y: 1},
                                Coord{x: 1, y: -1},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 2, y: -1},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 3, y: -2},
                                Coord{x: 3, y: -1},
                                Coord{x: 3, y: 0},
                                Coord{x: 3, y: 1},
                                Coord{x: 4, y: -2},
                                Coord{x: 4, y: -1},
                                Coord{x: 4, y: 0},
                                Coord{x: 4, y: 1},
                                Coord{x: 5, y: -1},
                                Coord{x: 5, y: 0},
        ]);
    }

    #[test]
    fn test_rectangles_2() {
        let pp = ParallelogramPixels{
            origin_side: (Coord {x: 1.0, y: 0.0}, Coord {x: 0.0, y: 2.0}),
            destination_side: (Coord {x: 5.0, y: 2.0}, Coord {x: 4.0, y: 4.0})
        };

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        assert_eq!(pixels.len(), 20);
        assert_eq!(pixels, vec![Coord{x: 0, y: 1},
                                Coord{x: 0, y: 2},
                                Coord{x: 1, y: 0},
                                Coord{x: 1, y: 1},
                                Coord{x: 1, y: 2},
                                Coord{x: 1, y: 3},
                                Coord{x: 2, y: 0},
                                Coord{x: 2, y: 1},
                                Coord{x: 2, y: 2},
                                Coord{x: 2, y: 3},
                                Coord{x: 3, y: 1},
                                Coord{x: 3, y: 2},
                                Coord{x: 3, y: 3},
                                Coord{x: 3, y: 4},
                                Coord{x: 4, y: 1},
                                Coord{x: 4, y: 2},
                                Coord{x: 4, y: 3},
                                Coord{x: 4, y: 4},
                                Coord{x: 5, y: 2},
                                Coord{x: 5, y: 3},
        ]);
    }
    #[test]
    fn test_parallelogram_pixels_bug() {
        // (61, 160) (60.99999999999999, 200), destination side: (57, 160) (56.99999999999999, 200)

        // let res = RiskMap::parallelogram_from_two_points(Coord{x: 0, y: 0}, Coord{x: 1, y: 0}, 300.0, 15.0);

        let pp = ParallelogramPixels::new(
            (Coord {y: 61.0, x: 160.0}, Coord {y: 60.99999999999999, x: 200.0}),
            (Coord {y: 57.0, x: 160.0}, Coord {y: 56.99999999999999, x: 200.0})
        );

        let mut pixels: Vec<Coord<i16>> = vec![];
        for coord in pp.iter() {
            pixels.push(coord);
        }

        // assert_eq!(pixels.len(), 20);
        assert_eq!(pixels, vec![
            Coord{x: 0, y: 1},
        ]);
    }

    #[test]
    fn test_neighbours_1() {
        let mut map = RiskMap{
            map: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
            m_per_pixel: 1.0,
            offset: 0
        };

        let mut coords: Vec<Coord<i16>> = vec![];

        for c in map.neighbours_within(Coord{x:0, y:0}, 2) {
            coords.push(c);
        }

        assert_eq!(coords, vec![
            Coord{x: 1, y: 0},
            Coord{x: 2, y: 0},
            Coord{x: 0, y: 1},
            Coord{x: 1, y: 1},
            Coord{x: 2, y: 1},
            Coord{x: 0, y: 2},
            Coord{x: 1, y: 2},
            Coord{x: 2, y: 2},
        ]);
    }

    #[test]
    fn test_neighbours_2() {
        let mut map = RiskMap{
            map: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
            m_per_pixel: 1.0,
            offset: 0
        };

        let mut coords: Vec<Coord<i16>> = vec![];

        for c in map.neighbours_within(Coord{x: 0, y: 0}, 0) {
            coords.push(c);
        }

        assert_eq!(coords, vec![
        ]);
    }

    #[test]
    fn test_neighbours_3() {
        let mut map = RiskMap{
            map: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
            m_per_pixel: 1.0,
            offset: 0
        };

        let mut coords: Vec<Coord<i16>> = vec![];

        for c in map.neighbours_within(Coord{x: 2, y: 2}, 1) {
            coords.push(c);
        }

        assert_eq!(coords, vec![
            Coord{x: 1, y: 1},
            Coord{x: 2, y: 1},
            Coord{x: 3, y: 1},
            Coord{x: 1, y: 2},
            Coord{x: 3, y: 2},
            Coord{x: 1, y: 3},
            Coord{x: 2, y: 3},
            Coord{x: 3, y: 3},
        ]);
    }

    #[test]
    fn test_neighbours_4() {
        let mut map = RiskMap{
            map: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
            m_per_pixel: 1.0,
            offset: 1
        };

        let mut coords: Vec<Coord<i16>> = vec![];

        for c in map.neighbours_within(Coord{x: 2, y: 2}, 1) {
            coords.push(c);
        }

        assert_eq!(coords, vec![
            Coord{x: 1, y: 1},
            Coord{x: 2, y: 1},
            Coord{x: 1, y: 2},
        ]);
    }

    #[test]
    fn test_all_points_1() {
        let mut map = RiskMap{
            map: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
            m_per_pixel: 1.0,
            offset: 0
        };

        let mut coords: Vec<Coord<i16>> = vec![];

        for c in map.all_points_iterator() {
            coords.push(c);
        }

        assert_eq!(coords, vec![
            Coord{x: 0, y: 0},
            Coord{x: 1, y: 0},
            Coord{x: 2, y: 0},
            Coord{x: 3, y: 0},
            Coord{x: 0, y: 1},
            Coord{x: 1, y: 1},
            Coord{x: 2, y: 1},
            Coord{x: 3, y: 1},
            Coord{x: 0, y: 2},
            Coord{x: 1, y: 2},
            Coord{x: 2, y: 2},
            Coord{x: 3, y: 2},
            Coord{x: 0, y: 3},
            Coord{x: 1, y: 3},
            Coord{x: 2, y: 3},
            Coord{x: 3, y: 3},
        ]);
    }

    #[test]
    fn test_all_points_2() {
        let mut map = RiskMap{
            map: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
            m_per_pixel: 1.0,
            offset: 1
        };

        let mut coords: Vec<Coord<i16>> = vec![];

        for c in map.all_points_iterator() {
            coords.push(c);
        }

        assert_eq!(coords, vec![
            Coord{x: 1, y: 1},
            Coord{x: 2, y: 1},
            Coord{x: 1, y: 2},
            Coord{x: 2, y: 2},
        ]);
    }
}

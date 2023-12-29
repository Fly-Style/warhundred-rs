#[cfg(test)]
mod tests;

pub mod grid {
    use grid::*;
    use lazy_static::lazy_static;
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap, HashSet};
    lazy_static! {
        static ref DIRECTIONS: Grid<(i32, i32)> = grid![
            [(1, 0), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)]
            [(1, 0), (1, -1), (0, -1), (-1, 0), (0, 1), (1, 1)]
        ];
    }

    // region Hex

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Hex {
        pub col: i32,
        pub row: i32,
        pub obstacle: bool,
        pub busy: bool,
    }

    impl PartialOrd for Hex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let cmp_result = self.col + self.row - other.col - other.row;
            if cmp_result == 0 {
                return Some(Ordering::Equal);
            }
            Some(if cmp_result > 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            })
        }
    }

    impl Ord for Hex {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl Hex {
        pub fn new_default(col: i32, row: i32) -> Self {
            Hex {
                col,
                row,
                obstacle: false,
                busy: false,
            }
        }

        pub fn new(col: i32, row: i32, obstacle: bool, busy: bool) -> Self {
            Hex {
                col,
                row,
                obstacle,
                busy,
            }
        }
    }

    // endregion Hex

    // region HexWithPriority

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct HexWithPriority<'a> {
        hex: &'a Hex,
        priority: i32,
    }

    impl PartialOrd for HexWithPriority<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.priority == other.priority {
                return Some(Ordering::Equal);
            }

            if self.priority < other.priority {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }

    impl Ord for HexWithPriority<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl<'a> HexWithPriority<'a> {
        pub fn new(hex: &'a Hex, priority: i32) -> Self {
            HexWithPriority { hex, priority }
        }
    }

    // endregion HexWithPriority

    pub struct Cube {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    pub struct FloatCube {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Debug)]
    pub struct HexGrid {
        grid: Grid<Hex>,
        width: usize,
        height: usize,
    }

    impl HexGrid {
        pub fn new(width: usize, height: usize, obstacles: &HashSet<(i32, i32)>) -> Self {
            let mut vec: Vec<Hex> = vec![];
            for i in 0..width as i32 {
                for j in 0..height as i32 {
                    vec.push(Hex {
                        col: i,
                        row: j,
                        obstacle: obstacles.contains(&(i, j)),
                        busy: false,
                    })
                }
            }
            HexGrid {
                grid: Grid::from_vec(vec, width),
                width,
                height,
            }
        }

        // Note: for test purposes only
        pub fn new_no_obstacles(width: usize, height: usize) -> Self {
            Self::new(width, height, &HashSet::<(i32, i32)>::new())
        }

        pub fn new_with_obstacles(width: usize, height: usize, obstacles: Vec<(usize, usize)>) -> Self {
            let mut grid = Self::new(width, height, &HashSet::<(i32, i32)>::new());
            obstacles.iter().for_each(|(col, row)| {
                grid.grid[*col][*row].obstacle = true;
            });
            grid
        }

        pub fn hex(self: &HexGrid, col: usize, row: usize) -> Option<&Hex> {
            self.grid.get(col, row)
        }

        pub fn are_neighbours(hex1: &Hex, hex2: &Hex) -> bool {
            let parity: usize = (hex1.row & 1) as usize;
            let directions = &DIRECTIONS[parity];
            for direction in directions.iter() {
                if hex1.col + direction.0 == hex2.col && hex1.row + direction.1 == hex2.row {
                    return true;
                }
            }
            false
        }

        pub fn pick_all_neighbours(self: &Self, hex: &Hex) -> Vec<&Hex> {
            (0..6)
                .map(|i: usize| -> Option<&Hex> { self.pick_neighbour(hex, i) })
                .filter(|hex| -> bool { hex.is_some() })
                .map(|hex| -> &Hex { hex.unwrap() })
                .collect()
        }

        pub fn pick_all_enterable_neighbours(self: &Self, hex: &Hex) -> Vec<&Hex> {
            (0..6)
                .map(|i: usize| -> Option<&Hex> { self.pick_neighbour(hex, i) })
                .filter(|hex| -> bool { hex.is_some() && !hex.unwrap().obstacle })
                .map(|hex| -> &Hex { hex.unwrap() })
                .collect()
        }

        fn pick_neighbour(self: &HexGrid, hex: &Hex, direction: usize) -> Option<&Hex> {
            let parity: usize = (hex.row & 1) as usize;
            let dir: (i32, i32) = *DIRECTIONS.get(parity, direction).unwrap();

            // check column existence
            if hex.col + dir.0 < 0 || hex.col + dir.0 >= self.width as i32 {
                return None;
            }

            // check row existence
            if hex.row + dir.1 < 0 || hex.row + dir.1 >= self.height as i32 {
                return None;
            }

            self.grid
                .get((hex.col + dir.0) as usize, (hex.row + dir.1) as usize)
        }

        pub fn distance(self: &Self, from: &Hex, to: &Hex) -> i32 {
            let cube_from = self.offset_to_cube(from);
            let cube_to = self.offset_to_cube(to);
            let x = (cube_from.x - cube_to.x).abs();
            let y = (cube_from.y - cube_to.y).abs();
            let z = (cube_from.z - cube_to.z).abs();
            self.cube_distance(x, y, z)
        }

        pub fn direct_path(self: &Self, from: &Hex, to: &Hex) -> Vec<&Hex> {
            let mut path: Vec<Cube> = vec![];

            let distance = self.distance(from, to);

            let cube_from = self.offset_to_cube(from);
            let cube_to = self.offset_to_cube(to);
            (0..distance + 1).for_each(|i| {
                let div = (1.0 / distance as f32) * i as f32;
                path.push(self.cube_round(self.cube_lerp(&cube_from, &cube_to, div)))
            });

            path.iter().map(|cube| self.cube_to_offset(cube)).collect()
        }

        pub fn smart_path<'a>(self: &'a Self, from: &'a Hex, to: &'a Hex) -> Vec<&Hex> {
            let mut frontier: BinaryHeap<HexWithPriority> = BinaryHeap::new();
            let mut path: Vec<&Hex> = vec![];
            let mut came_from: HashMap<&Hex, &Hex> = HashMap::new();
            let mut cost_map: HashMap<&Hex, i32> = HashMap::new();
            let mut end_reached = false;

            frontier.push(HexWithPriority::new(from, 0));

            println!("Start: {:?}", from);
            println!("End: {:?}", to);
            while frontier.len() > 0 {
                let candidate = frontier.pop().unwrap();
                if candidate.hex.eq(to) {
                    end_reached = true;
                    break;
                }

                let neighbours = self.pick_all_enterable_neighbours(candidate.hex);
                neighbours.iter().for_each(|_next| {
                    let next = *_next;
                    let new_cost = self.distance(next, to);
                    if new_cost < 0 {
                        panic!("Negative cost");
                    }
                    if !cost_map.contains_key(next) || *cost_map.get(next).unwrap() < new_cost {
                        cost_map.insert(next, new_cost);
                        came_from.insert(next, candidate.hex);
                        frontier.push(HexWithPriority::new(next, new_cost));
                    }
                });
            }

            println!("{:?}", came_from);

            assert!(end_reached, "A* algorithm didn't reach the end.");

            let mut current = to;
            while current != from {
                path.push(current);
                match came_from.get(current) {
                    Some(next) => {
                        current = *next;
                    }
                    None => {
                        break;
                    }
                }
            }
            path.reverse();

            return path;
        }

        // TODO: decompose this functions into separate trait later.
        fn offset_to_cube(self: &Self, hex: &Hex) -> Cube {
            // As we are using odd-r offset hexagonal game.grid, standard offset is -1.
            let offset = -1;

            let x = hex.col - ((hex.row + offset * (hex.row & 1)) / 2);
            let y = hex.row;
            let z = -x - y;
            Cube { x, y, z }
        }

        fn cube_to_offset(self: &Self, cube: &Cube) -> &Hex {
            // As we are using odd-r offset hexagonal game.grid, standard offset is -1.
            let offset = -1;

            let col: usize = (cube.x + ((cube.y + offset * (cube.y & 1)) / 2)) as usize;
            let row: usize = cube.y as usize;
            return &self.grid[col][row];
        }

        fn cube_distance(self: &Self, x: i32, y: i32, z: i32) -> i32 {
            (x + y + z) / 2
        }

        /// Cube hex linear interpolation algorithm
        fn cube_lerp(&self, a: &Cube, b: &Cube, t: f32) -> FloatCube {
            FloatCube {
                x: a.x as f32 + (b.x as f32 - a.x as f32) * t,
                y: a.y as f32 + (b.y as f32 - a.y as f32) * t,
                z: a.z as f32 + (b.z as f32 - a.z as f32) * t,
            }
        }

        fn cube_round(&self, cube: FloatCube) -> Cube {
            let mut rx = cube.x.round();
            let mut ry = cube.y.round();
            let mut rz = cube.z.round();

            let x_diff = (rx - cube.x).abs();
            let y_diff = (ry - cube.y).abs();
            let z_diff = (rz - cube.z).abs();

            if x_diff > y_diff && x_diff > z_diff {
                rx = -ry - rz;
            } else if y_diff > z_diff {
                ry = -rx - rz;
            } else {
                rz = -rx - ry
            }

            Cube {
                x: rx as i32,
                y: ry as i32,
                z: rz as i32,
            }
        }
    }
}

#[cfg(test)]
mod tests;

pub mod grid {
    use grid::*;
    use lazy_static::lazy_static;
    use std::cmp::Ordering;
    use std::collections::HashSet;

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

    // endregion Hex

    // region HexWithPriority

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct HexWithPriority {
        hex: Hex,
        priority: u32,
    }

    impl PartialOrd for HexWithPriority {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.priority == other.priority {
                return Some(Ordering::Equal);
            }

            if self.priority > 0 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }

    impl Ord for HexWithPriority {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    // endregion HexWithPriority

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

            self.grid.get((hex.col + dir.0) as usize, (hex.row + dir.1) as usize)
        }

        pub fn pick_all_neighbours(self: &Self, hex: &Hex) -> Vec<&Hex> {
            (0..5)
                .map(|i: usize| -> Option<&Hex> { self.pick_neighbour(hex, i) })
                .filter(|hex| -> bool { hex.is_some() })
                .map(|hex| -> &Hex { hex.unwrap() })
                .collect()
        }
    }
}

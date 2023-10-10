#[cfg(test)]
mod tests;

pub mod grid {
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use grid::Grid;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref DIRECTIONS: Grid<(i32, i32)> = Grid::<(i32, i32)>::from_vec(
            vec![(1, 0), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1),
                 (1, 0), (1, -1), (0, -1), (-1, 0), (0, 1), (1, 1)],
            2);
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
            Some(if cmp_result > 0 { Ordering::Greater } else { Ordering::Less })
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
            };
            if self.priority > 0 { Some(Ordering::Greater) } else { Some(Ordering::Less) }
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
            HexGrid { grid: Grid::from_vec(vec, width) }
        }

        // Note: for test purposes only
        pub fn new_no_obstacles(width: usize, height: usize) -> Self {
            Self::new(width, height, &HashSet::<(i32, i32)>::new())
        }

        fn pick_neighbours(hex: &Hex) -> &Hex {
            todo!()
        }
    }
}
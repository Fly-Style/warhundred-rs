use std::cmp::Ordering;
use crate::grid::{Hex, HexGrid};

// region Hex

#[test]
fn hex_ordering_test() {
    let hex = Hex {col : 1, row : 2, obstacle: false, busy : false};
    let hex2: Hex = Hex {col : 0, row : 2, obstacle: false, busy : false};
    assert_eq!(hex.partial_cmp(&hex2).unwrap(), Ordering::Greater);

    let hex3: Hex = Hex {col : 2, row : 2, obstacle: false, busy : false};
    assert_eq!(hex.partial_cmp(&hex3).unwrap(), Ordering::Less);
}

// endregion Hex

// region HexGrid

#[test]
fn when_hexgrid_creating_then_success() {
    let _hex_grid = HexGrid::new_no_obstacles(1, 2);
}

// endregion HexGrid
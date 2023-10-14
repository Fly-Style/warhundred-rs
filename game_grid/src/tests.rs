use crate::grid::{Hex, HexGrid};
use std::cmp::Ordering;

// region Hex

#[test]
fn when_hex_ordered_then_ordered_correctly() {
    let hex = Hex {
        col: 1,
        row: 2,
        obstacle: false,
        busy: false,
    };
    let hex2: Hex = Hex {
        col: 0,
        row: 2,
        obstacle: false,
        busy: false,
    };
    assert_eq!(hex.partial_cmp(&hex2).unwrap(), Ordering::Greater);

    let hex3: Hex = Hex {
        col: 2,
        row: 2,
        obstacle: false,
        busy: false,
    };
    assert_eq!(hex.partial_cmp(&hex3).unwrap(), Ordering::Less);
}

// endregion Hex

// region HexGrid

#[test]
fn when_hexgrid_picks_neighbours_then_success() {
    let hex_grid = HexGrid::new_no_obstacles(4, 4);
}

// endregion HexGrid

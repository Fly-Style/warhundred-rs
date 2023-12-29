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
fn hexgrid_test_pick_neighbour() {
    let _hex_grid = HexGrid::new_no_obstacles(4, 4);
}

#[test]
fn hexgrid_test_pick_all_neighbours() {
    let hex_grid = HexGrid::new_no_obstacles(4, 4);

    let base_hex = Hex::new_default(2, 2);
    let neighbours = hex_grid.pick_all_neighbours(&base_hex);
    assert_eq!(neighbours.len(), 6);

    let expected_neighbors = vec![
        Hex::new_default(3, 2),
        Hex::new_default(2, 1),
        Hex::new_default(1, 1),
        Hex::new_default(1, 2),
        Hex::new_default(1, 3),
        Hex::new_default(2, 3)];

    (0..neighbours.len()).for_each(|i| {
        assert_eq!(neighbours[i], &expected_neighbors[i]);
    });
}

#[test]
fn hexgrid_test_direct_path_common() {
    let hex_grid = HexGrid::new_no_obstacles(4, 4);
    let from = Hex::new_default(1, 1);
    let to = Hex::new_default(3, 3);

    let path = hex_grid.direct_path(&from, &to);
    assert_eq!(path.len(), 4);

    let expected_path = vec![
        Hex::new_default(1, 1),
        Hex::new_default(2, 2),
        Hex::new_default(3, 2),
        Hex::new_default(3, 3),
    ];
    (0..path.len()).for_each(|i| {
        assert_eq!(path[i], &expected_path[i]);
    });
}

#[test]
fn hexgrid_test_direct_path_neighbors() {
    let hex_grid = HexGrid::new_no_obstacles(4, 4);
    let from = Hex::new_default(0, 0);
    let to = Hex::new_default(1, 0);

    let path = hex_grid.direct_path(&from, &to);
    assert_eq!(path.len(), 2);

    let expected_path = vec![
        Hex::new_default(0, 0),
        Hex::new_default(1, 0),
    ];
    (0..path.len()).for_each(|i| {
        assert_eq!(path[i], &expected_path[i]);
    });
}

#[test]
fn hexgrid_test_smart_path_with_obstacles() {
    let hex_grid = HexGrid::new_with_obstacles(5, 5, vec![(1, 3), (2, 3), (3, 2)]);
    let from = Hex::new_default(0, 3);
    let to = Hex::new_default(4, 3);

    let path = hex_grid.smart_path(&from, &to);
    assert_eq!(path.len(), 6);

    let expected_path = vec![
            Hex::new_default(1, 2),
            Hex::new_default(2, 2),
            Hex::new_default(2, 1),
            Hex::new_default(3, 1),
            Hex::new_default(4, 2),
            Hex::new_default(4, 3)
    ];
    (0..path.len()).for_each(|i| {
        assert_eq!(path[i], &expected_path[i]);
    });
}

// endregion HexGrid

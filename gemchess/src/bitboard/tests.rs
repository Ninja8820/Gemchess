use crate::{Bitboard, Tile};


#[test]
fn set_get_bit() {
    let tile = Tile::new_index(5).unwrap();
    let mut bb = Bitboard::EMPTY;

    bb.set_bit(tile, true);
    assert!(bb.get_bit(tile));

    bb.set_bit(tile, false);
    assert!(!bb.get_bit(tile));
}

#[test]
fn bitwise_and_or() {
    let t1 = Tile::new_unchecked(3);
    let t2 = Tile::new_unchecked(6);

    let bb1 = t1.mask();
    let bb2 = t2.mask();

    let bb_or = bb1 | bb2;
    assert!(bb_or.get_bit(t1));
    assert!(bb_or.get_bit(t2));

    let bb_and = bb1 & bb2;
    assert_eq!(bb_and.count_ones(), 0);
}

#[test]
fn not_operation() {
    let tile = Tile::new_index(2).unwrap();
    let bb = tile.mask();
    let bb_not = !bb;

    assert!(!bb_not.get_bit(tile));
    assert_eq!(bb_not.count_ones(), 63);
}

#[test]
fn shift_operations() {
    let bb = Bitboard::new(0b1);
    let shifted = bb << 3;
    assert_eq!(shifted, 0b1000);
    let shifted_back = shifted >> 3;
    assert_eq!(shifted_back, 0b1);
}

#[test]
fn to_bit_singleton() {
    let tile = Tile::new_index(10).unwrap();
    let bb = tile.mask();
    assert_eq!(bb.to_bit(), Some(tile));

    let combined = bb | Tile::D2.mask();
    assert_eq!(combined.to_bit(), None);
}

#[test]
fn iteration_over_bits() {
    let mut bb = Bitboard::EMPTY;
    let t1 = Tile::new_index(7).unwrap();
    let t2 = Tile::new_index(15).unwrap();
    bb.set_bit(t1, true);
    bb.set_bit(t2, true);

    let mut collected: Vec<u8> = bb.clone().map(|t| t.shift()).collect();
    collected.sort();
    assert_eq!(collected, vec![7, 15]);
}

#[test]
fn some_and_none() {
    let bb = Bitboard::EMPTY;
    assert!(bb.is_empty());
    assert!(!bb.some());

    let bb2 = Bitboard::new(1u64);
    assert!(bb2.some());
    assert!(!bb2.is_empty());
}
pub fn between(a: u8, b: u8) -> u64 {
    if a == b {
        return 0;
    }

    let (ax, ay) = (a % 8, a / 8);
    let (bx, by) = (b % 8, b / 8);

    let dx = bx as i8 - ax as i8;
    let dy = by as i8 - ay as i8;

    // Rank, file or diagonal aligned
    if !(dx == 0 || dy == 0 || dx.abs() == dy.abs()) {
        return 0;
    }

    let step_x = dx.signum();
    let step_y = dy.signum();

    let mut mask = 0;
    let mut x = ax as i8 + step_x;
    let mut y = ay as i8 + step_y;
    while (x, y) != (bx as i8, by as i8) {
        mask |= 1u64 << (y * 8 + x);
        x += step_x;
        y += step_y;
    }

    mask
}

pub fn neighbours(t: u8) -> u64 {
    let x = (t % 8) as i8;
    let y = (t / 8) as i8;

    let mut mask = 0u64;
    for dx in -1i8..=1 {
        for dy in -1i8..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let (nx, ny) = (x + dx, y + dy);
            if (0..8).contains(&nx) && (0..8).contains(&ny) {
                mask |= 1u64 << (ny * 8 + nx);
            }
        }
    }

    mask
}

pub fn knight(t: u8) -> u64 {
    let x = (t % 8) as i8;
    let y = (t / 8) as i8;

    const OFFSETS: [(i8, i8); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    let mut mask = 0u64;
    for (dx, dy) in OFFSETS {
        let (nx, ny) = (x + dx, y + dy);
        if (0..8).contains(&nx) && (0..8).contains(&ny) {
            mask |= 1u64 << (ny * 8 + nx);
        }
    }
    mask
}

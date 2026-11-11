use std::fmt::{Error, Write};

const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn relevant_mask(t: u8, dirs: &[(i8, i8); 4]) -> u64 {
    let (x0, y0) = (t as i8 % 8, t as i8 / 8);

    let mut mask = 0u64;
    for &(dx, dy) in dirs {
        let (mut x, mut y) = (x0 + dx, y0 + dy);

        loop {
            let x_ok = dx == 0 || (1..7).contains(&x);
            let y_ok = dy == 0 || (1..7).contains(&y);
            if !x_ok || !y_ok {
                break;
            }

            mask |= 1u64 << (y * 8 + x);
            x += dx;
            y += dy;
        }
    }
    mask
}

fn sliding_attacks(t: u8, blockers: u64, dirs: &[(i8, i8); 4]) -> u64 {
    let (x0, y0) = (t as i8 % 8, t as i8 / 8);

    let mut attacks = 0u64;
    for &(dx, dy) in dirs {
        let (mut x, mut y) = (x0 + dx, y0 + dy);

        while (0..8).contains(&x) && (0..8).contains(&y) {
            let curr = (y * 8 + x) as u8;
            attacks |= 1u64 << curr;
            if blockers & (1u64 << curr) != 0 {
                break;
            }

            x += dx;
            y += dy;
        }
    }
    attacks
}

fn subsets(mask: u64) -> Vec<u64> {
    let mut out = Vec::with_capacity(1 << mask.count_ones());
    let mut subset = 0u64;
    loop {
        out.push(subset);
        subset = subset.wrapping_sub(mask) & mask;
        if subset == 0 {
            break;
        }
    }
    out
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn sparse_u64(&mut self) -> u64 {
        self.next_u64() & self.next_u64() & self.next_u64()
    }
}

fn find_magic(t: u8, mask: u64, dirs: &[(i8, i8); 4], rng: &mut Rng) -> (u64, u8, Vec<u64>) {
    let bits = mask.count_ones();
    let shift = 64 - bits as u8;
    let occupancies = subsets(mask);
    let real_attacks: Vec<u64> = occupancies
        .iter()
        .map(|&occ| sliding_attacks(t, occ, dirs))
        .collect();

    loop {
        let magic = rng.sparse_u64();
        if (mask.wrapping_mul(magic) >> 56).count_ones() < 6 {
            continue;
        }

        let mut table: Vec<Option<u64>> = vec![None; 1 << bits];
        let mut ok = true;
        for (i, &occ) in occupancies.iter().enumerate() {
            let idx = (occ.wrapping_mul(magic) >> shift) as usize;
            match table[idx] {
                None => table[idx] = Some(real_attacks[i]),
                Some(existing) if existing == real_attacks[i] => {}
                Some(_) => {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            return (
                magic,
                shift,
                table.into_iter().map(|v| v.unwrap_or(0)).collect(),
            );
        }
    }
}

pub fn rook_magic_body() -> Result<String, Error> {
    magic_body("ROOK", &ROOK_DIRS, 0x1234_5678_9abc_def0)
}

pub fn bishop_magic_body() -> Result<String, Error> {
    magic_body("BISHOP", &BISHOP_DIRS, 0x0fed_cba9_8765_4321)
}

fn magic_body(name: &str, dirs: &[(i8, i8); 4], seed: u64) -> Result<String, Error> {
    let mut rng = Rng::new(seed);
    let mut entries = Vec::with_capacity(64);
    let mut flat: Vec<u64> = Vec::new();

    for t in 0u8..64 {
        let mask = relevant_mask(t, dirs);
        let (magic, shift, table) = find_magic(t, mask, dirs, &mut rng);
        let offset = flat.len() as u32;
        entries.push((mask, magic, shift, offset));
        flat.extend(table);
    }

    let mut out = String::new();

    writeln!(
        out,
        "pub const {name}_MAGICS: TileMap<MagicEntry> = TileMap::new(["
    )?;
    for (mask, magic, shift, offset) in &entries {
        writeln!(
            out,
            "    MagicEntry {{ mask: Bitboard::new(0x{mask:016x}), magic: Bitboard::new(0x{magic:016x}), shift: {shift}, offset: {offset} }},"
        )?;
    }
    writeln!(out, "]);\n")?;

    writeln!(
        out,
        "pub const {name}_ATTACKS: [Bitboard; {}] = [",
        flat.len()
    )?;
    for bb in &flat {
        writeln!(out, "    Bitboard::new(0x{bb:016x}),")?;
    }
    writeln!(out, "];")?;

    Ok(out)
}

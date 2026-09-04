use std::{
    fmt::{Error, Write as _},
    path::Path,
};

pub fn tile_map_body(const_name: &str, mask_fn: impl Fn(u8) -> u64) -> Result<String, Error> {
    let mut code = String::with_capacity(64 * 24 + 128);

    writeln!(
        code,
        "pub const {const_name}: TileMap<Bitboard> = TileMap::new(["
    )?;

    for t in 0..64 {
        writeln!(code, "\tBitboard::new(0x{:016x}),", mask_fn(t))?;
    }

    write!(code, "]);")?;

    Ok(code)
}

pub fn tile_map_2d_body(
    const_name: &str,
    mask_fn: impl Fn(u8, u8) -> u64,
) -> Result<String, Error> {
    let mut code = String::with_capacity(64 * 64 * 24 + 128);

    writeln!(
        code,
        "pub const {const_name}: TileMap<TileMap<Bitboard>> = TileMap::new(["
    )?;

    for a in 0u8..64 {
        write!(code, "\tTileMap::new([")?;
        for b in 0u8..64 {
            write!(code, "\t\tBitboard::new(0x{:016x}), ", mask_fn(a, b))?;
        }
        writeln!(code, "\t]),")?;
    }
    write!(code, "]);")?;

    Ok(code)
}

pub fn write_generated_file(module: &str, body: &str) -> std::fmt::Result {
    let mut code = String::with_capacity(body.len() + 256);

    writeln!(
        code,
        "use crate::{{Bitboard, magics::tilemap::TileMap, MagicEntry}};\n"
    )?;

    code.push_str(body);

    let out_path = Path::new("gemchess/src/magics").join(format!("{module}.rs"));
    std::fs::write(&out_path, code)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));

    println!("wrote {}", out_path.display());

    Ok(())
}

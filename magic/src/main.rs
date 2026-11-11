mod format;
mod magic;
mod masks;

use clap::{Parser, ValueEnum};

use crate::{
    format::{tile_map_2d_body, tile_map_body, write_generated_file},
    magic::{bishop_magic_body, rook_magic_body},
    masks::{between, knight, neighbours},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Modules {
    Between,
    King,
    Knight,
    Rook,
    Bishop,
}

#[derive(Parser)]
struct Args {
    module: Modules,
}

fn main() -> std::fmt::Result {
    let args = Args::parse();

    match args.module {
        // Lookups
        Modules::Between => write_generated_file("between", &tile_map_2d_body("BETWEEN", between)?),
        Modules::King => {
            write_generated_file("king_attacks", &tile_map_body("KING_ATTACKS", neighbours)?)
        }
        Modules::Knight => {
            write_generated_file("knight_attacks", &tile_map_body("KNIGHT_ATTACKS", knight)?)
        }

        // Magics
        Modules::Rook => write_generated_file("rook_magics", &rook_magic_body()?),
        Modules::Bishop => write_generated_file("bishop_magics", &bishop_magic_body()?),
    }
}

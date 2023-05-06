use crate::{board::Player, location::Location};

pub struct Bitfields {
    king: u64,
    queen: u64,
    knight: [u64; 2],
    pawn: [u64; 8],
    rook: [u64; 2],
    bishop: [u64; 2],
}

impl Bitfields {
    fn new(player: Player) -> Bitfields {
        let bitfield = |p: Option<Location>| {
            p.map(Location::pos)
                .unwrap_or_default()
        };

        let king = bitfield(player.king);
        let queen = bitfield(player.queen);
        let pawn = player.pawn.map(bitfield);
        let knight = player.knight.map(bitfield);
        let bishop = player.bishop.map(bitfield);
        let rook = player.rook.map(bitfield);

        Bitfields {
            king,
            queen,
            knight,
            pawn,
            rook,
            bishop,
        }
    }
}

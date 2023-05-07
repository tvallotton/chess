use crate::{board::Player, location::Location};

pub struct Bitfields {
    pub royalty: [u64; 2],
    pub knight: [u64; 2],
    pub pawn: [u64; 8],
    pub rook: [u64; 2],
    pub bishop: [u64; 2],
}

impl Bitfields {
    pub fn new(player: &Player) -> Bitfields {
        let bitfield = |p: Option<Location>| {
            p.map(Location::pos)
                .unwrap_or_default()
        };

        let royalty = player.royalty.map(bitfield);
        let pawn = player.pawn.map(bitfield);
        let knight = player.knight.map(bitfield);
        let bishop = player.bishop.map(bitfield);
        let rook = player.rook.map(bitfield);

        Bitfields {
            royalty,
            knight,
            pawn,
            rook,
            bishop,
        }
    }
}

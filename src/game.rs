use crate::board::Board;
use crate::minimax::Node;
use crate::moves::Move;
use crate::opt::Settings;
use crate::parameters::Params;
use crate::piece::Color;

use serde_json::from_str;

use std::fmt::Display;

use tap::Pipe;
#[derive(Clone)]
pub struct Game {
    turn: Color,
    node: Node,
    opt: Settings,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.node.board)?;
        writeln!(f, "white heuristic:    {}", self.white_heuristic())?;
        writeln!(f, "black heuristic:    {}", self.black_heuristic())?;
        writeln!(f, "absolute heuristic: {}", self.absolute_heuristic())
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: Color::White,
            node: Node::default(),
            opt: include_str!("../settings.json")
                .pipe(from_str)
                .unwrap(),
        }
    }

    fn params(&self) -> &Params {
        match self.turn {
            Color::White => &self.opt.white_params,
            Color::Black => &self.opt.black_params,
        }
    }
    fn board(&self) -> Board {
        self.node.board
    }
    pub fn play(&mut self) -> bool {
        log::info!("playing for: {:?}", self.node.turn);
        if let Some((mov, val)) = self.clone().minimax() {
            log::info!("move heuristic: {val}");
            self.node.board = self.node.board.apply(mov);

            self.turn = self.turn.opposite();
            self.node.turn = self.turn;
            return true;
        }
        false
    }

    fn minimax(&mut self) -> Option<(Move, f32)> {
        let (mut black, mut white) = (0.0, 0.0);
        let moves = self
            .node
            .children_with_moves(self.params())
            .map(|(node, mov)| {
                let minimax = node.minimax(
                    self.params(),
                    self.opt.max_depth,
                    self.turn,
                    &mut black,
                    &mut white,
                );

                log::debug!("CHILD: {}\nminimax: {minimax}", node.board);
                (mov, minimax)
            });
        if self.turn == Color::White {
            moves.max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        } else {
            moves.min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        }
    }

    pub fn winner(&self) {
        let h = self.absolute_heuristic();
        if h == 0.0 {
            println!("tie")
        } else if h > 0.0 {
            println!("white")
        } else {
            println!("black")
        }
    }
    fn white_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(&self.opt.white_params, false)
    }
    fn black_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(&self.opt.black_params, false)
    }
    fn absolute_heuristic(&self) -> f32 {
        self.node
            .board
            .heuristic(&self.opt.absolute_params, true)
    }
}

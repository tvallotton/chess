use std::{fs::*, io};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::board::Board;
use crate::parameters::Params;
use crate::piece::*;

fn join(params: &[Params]) -> Params {
    let mut out = params[0].clone();
    out.attacked = 0.0;
    out.available_moves = 0.0;
    out.defended = 0.0;
    out.castle_kingside = 0.0;
    out.castle_queenside = 0.0;
    out.mov_value = 0.0;
    let len = params.len() as f32;
    for par in params {
        out.attacked += par.attacked / len;
        out.available_moves += par.available_moves / len;
        out.defended += par.defended / len;
        out.castle_kingside += par.castle_kingside / len;
        out.castle_queenside += par.castle_queenside / len;
        out.mov_value += par.mov_value / len;
    }
    out
}

const POPULATION: usize = 100;


#[test]
fn foo() {
    for i in 0.. {
        train(); 
        println!("finished round: {i}"); 
    }
}
fn train() {
    use rayon::*;
    pretty_env_logger::init();
    let pop = load().unwrap();
    let mut new_pop = vec![];

    pop.par_iter()
        .map(|params| {
            let mut board = Board::default();

            let (black, white) = params.split();
            board.play_random(&params);
            board.play_random(&params);

            for i in 0..30 {
                board.play_with(&white);
                board.play_with(&black);
            }

            println!("{}", board.heuristic(&params));
            if board
                .heuristic(&params)
                .is_sign_positive()
            {
                white
            } else {
                black
            }
        })
        .collect_into_vec(&mut new_pop);
    save(&new_pop);
}
pub fn save(params: &[Params]) {
    let name = format!("./train/params.json");
    let file = File::create(name).unwrap();
    let par = join(params);
    serde_json::to_writer(file, &par).unwrap();
}
pub fn load() -> io::Result<Vec<Params>> {
    let mut out = vec![];

    let name = format!("./train/params.json");
    let file = read_to_string(name)?;
    let params: Params = serde_json::from_str(&file).unwrap();

    for i in 0..POPULATION {
        out.push(params.clone())
    }

    Ok(out)
}

pub fn populate() -> io::Result<()> {
    for i in 0..1 {
        let name = format!("./train/params_{i}.json");

        let mut file = File::create(name)?;

        serde_json::to_writer(file, &Params::default());
    }
    Ok(())
}

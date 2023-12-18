use crate::params::Params;

use super::common::*;
use rand::prelude::*;

pub fn grow_stat(program: &mut Program, depth: usize, params: &Params, rand: &mut StdRng) -> bool {
    if program.len() >= MAX_LEN || depth > params.depth {
        return false;
    }
    let stat = *TOKEN_STAT.choose(rand).unwrap();
    program.push(Token::Kw(stat));
    match stat {
        Funcs::OUTPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            program.push(reg);
        }
        Funcs::INPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            program.push(reg);
        }
        _ => panic!("{:?} is not a stat (or implementation missing)", stat),
    }
    println!("{:?}", program);
    return true;
}
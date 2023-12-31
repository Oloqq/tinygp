use crate::params::Params;

use super::common::*;
use rand::prelude::*;

pub fn grow_stat(program: &mut Program, depth: usize, params: &Params, rand: &mut StdRng) -> bool {
    if program.len() >= MAX_LEN || depth > params.depth {
        return false;
    }
    let stat = rand.gen();
    program.push(Token::Stat(stat));
    match stat {
        Stat::OUTPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            program.push(reg);
        }
        Stat::INPUT => {
            let regnum = rand.gen_range(0, params.memsize);
            let reg = Token::Reg(regnum);
            program.push(reg);
        },
        _ => todo!()
    }
    log::trace!("grew into {:?}", program);
    return true;
}
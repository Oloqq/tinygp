use super::common::*;
use crate::params::Params;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub fn rand_reg(params: &Params, rand: &mut StdRng) -> Token {
    Token::Reg(rand.gen_range(0, params.memsize))
}

pub fn rand_const(params: &Params, rand: &mut StdRng) -> Token {
    Token::Expr(Expr::Num(
        rand.gen_range(params.growing.min_const, params.growing.max_const),
    ))
}

pub fn grow_expr(params: &Params, rand: &mut StdRng) -> Vec<Token> {
    let mut code = vec![];

    if rand.gen_bool(params.growing.p_expression_plug) {
        if rand.gen_bool(params.growing.p_prefer_reg_over_num) {
            code.push(rand_reg(params, rand))
        } else {
            code.push(rand_const(params, rand))
        }
    } else {
        let items = &params.growing.d_expr;
        let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        let e: Expr = items[dist2.sample(rand)].0;
        println!("{e:?}");
        match e {
            Expr::Reg(_) => {
                println!("substituting");
                code.push(rand_reg(params, rand))
            }
            Expr::Num(_) => code.push(rand_const(params, rand)),
            _ => {
                code.push(Token::Expr(e));
                assert!(!matches!(e, Expr::Reg(_)));
                for _ in 0..e.argnum() {
                    code.append(&mut grow_expr(params, rand));
                }
            },
        }
    }
    code
}

pub fn grow_stat(
    size_left: usize,
    params: &Params,
    rand: &mut StdRng,
) -> Vec<Token> {
    let stat: Stat = rand.gen();
    let mut code: Vec<Token> = vec![];
    code.push(Token::Stat(stat));
    match stat {
        Stat::INPUT => {
            code.push(rand_reg(params, rand));
        }
        Stat::OUTPUT => {
            let mut expr = grow_expr(params, rand);
            if code.len() + expr.len() > size_left {
                return vec![];
            }
            code.append(&mut expr);
        }
        Stat::LOAD => {
            code.push(rand_reg(params, rand));
            let mut expr = grow_expr(params, rand);
            if code.len() + expr.len() > size_left {
                return vec![];
            }
            code.append(&mut expr);
        }
        _ => {
            log::error!("growing logic unfinished");
            return vec![];
        }
    }
    return if size_left > code.len() { code } else { vec![] };
}

pub fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    let mut program: Program = Vec::with_capacity(50);
    program.append(&mut vec![Token::Stat(Stat::INPUT), Token::Reg(0)]);
    program.append(&mut grow_stat(params.max_size, params, rand));
    program.append(&mut grow_stat(params.max_size, params, rand));
    program.append(&mut vec![Token::Stat(Stat::OUTPUT), Token::Reg(0)]);
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    // const ANY_PARAMS: Params = Params {
    //     seed: 0,
    //     memsize: 3,
    //     popsize: 10,
    //     depth: 3,
    //     crossover_prob: 0.9,
    //     pmut_per_node: 0.05,
    //     tournament_size: 2,
    //     acceptable_error: 0.1,
    // };

    #[test]
    #[ignore]
    fn test_random_indiv() {
        let params = Params {
            ..Default::default()
        };
        let mut rand = StdRng::seed_from_u64(0);
        let prog = create_random_indiv(&params, &mut rand);
        assert!(prog.len() > 4);
        assert!(matches!(prog[0], Token::Stat(Stat::INPUT)));
        assert!(matches!(prog[prog.len() - 2], Token::Stat(Stat::OUTPUT)));
    }

    #[test]
    // #[ignore]
    fn test_rand_expr() {
        let mut rand = StdRng::seed_from_u64(0);
        for _ in 0..1000 {
            println!("{:?}", grow_expr(&Params::default(), &mut rand));
        }
        // assert!(false);
    }
}

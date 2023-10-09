use std::{error::Error, fmt::Display};

pub type Case = Vec<f32>;

pub struct Params {
    pub seed: u64,
    pub min_random: f32,
    pub max_random: f32,
    pub varnumber: i32,
    pub const_numbers: i32,
    pub max_len: usize,
    pub popsize: usize,
    pub depth: usize,
    pub crossover_prob: f32,
    pub pmut_per_node: f32,
    pub tournament_size: usize,
}

impl Params {
    pub fn from_string(data: String) -> Result<(Params, Vec<Case>), Box<dyn Error>> {
        let lines: Vec<&str> = data.split('\n').collect();
        // println!("line {:?}", lines);
        let header: Vec<&str> = lines[0].trim().split([' ', '\t']).collect();
        let varnumber: i32 = header[0].parse()?;
        let random_number: i32 = header[1].parse()?;
        let min_random: f32 = header[2].parse()?;
        let max_random: f32 = header[3].parse()?;
        let num_cases: usize = header[4].parse()?;

        let mut cases: Vec<Case> = Vec::with_capacity(num_cases);
        for i in 0..num_cases {
            let tokens: Vec<&str> = lines[i + 1]
                .trim()
                .split([' ', '\t'])
                .filter(|t| !t.is_empty())
                .collect();
            let case: Case = tokens.iter().map(|t| t.parse().unwrap()).collect();
            cases.push(case);
        }

        Ok((
            Params {
                seed: 5,
                min_random,
                max_random,
                varnumber,
                const_numbers: random_number,
                ..Default::default()
            },
            cases,
        ))
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            min_random: Default::default(),
            max_random: Default::default(),
            varnumber: Default::default(),
            const_numbers: Default::default(),
            max_len: 10000,
            popsize: 100000,
            depth: 5,
            crossover_prob: 0.9,
            pmut_per_node: 0.05,
            tournament_size: 2,
        }
    }
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "SEED={}\nMAX_LEN={})
POPSIZE={}
DEPTH={})
CROSSOVER_PROB={})
PMUT_PER_NODE={})
MIN_RANDOM={})
MAX_RANDOM={})
TSIZE={})
----------------------------------\n",
                self.seed,
                self.max_len,
                self.popsize,
                self.depth,
                self.crossover_prob,
                self.pmut_per_node,
                self.min_random,
                self.max_random,
                self.tournament_size
            )
            .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::params::Params;

    #[test]
    fn test_read_params() {
        let (param, cases) = match Params::from_string(
            "1 100 -5 5 10
0 1
1	2
2	3
3	4
4   5
5   6
6   7
7   8
8   9
9   10"
                .to_owned(),
        ) {
            Ok(p) => p,
            Err(_) => panic!("Read problem failed"),
        };

        assert_eq!(param.seed, 5);
        assert_eq!(param.min_random, -5.0);
        assert_eq!(param.max_random, 5.0);

        assert_eq!(cases.len(), 10);
    }
}

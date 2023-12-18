use std::{error::Error, fmt::Display};

pub type Case = (Vec<f32>, Vec<f32>);

pub struct Params {
    pub seed: u64,
    pub memsize: usize,
    pub popsize: usize,
    pub depth: usize,
    pub crossover_prob: f32,
    pub pmut_per_node: f32,
    pub tournament_size: usize,
    pub acceptable_error: f32,
}

impl Params {
    pub fn from_string(data: String) -> Result<(Params, Vec<Case>), Box<dyn Error>> {
        let lines: Vec<&str> = data.split('\n').collect();
        let header: Vec<&str> = lines[0].trim().split([' ', '\t']).collect();
        let memsize: usize = header[0].parse()?;
        let separator: &str = header[1];
        let num_cases: usize = header[2].parse()?;

        let mut cases: Vec<Case> = Vec::with_capacity(num_cases);
        for i in 0..num_cases {
            let tokens: Vec<&str> = lines[i + 1]
                .trim()
                .split([' ', '\t'])
                .filter(|t| !t.is_empty())
                .collect();
            let split_pos = tokens.iter().position(|&t| t == separator).expect("No pipe '|' found in the input");
            let (inputs, pipe_and_outputs) = tokens.split_at(split_pos);
            let outputs = &pipe_and_outputs[1..];
            let inputs = inputs
                .iter()
                .map(|t| t.parse().unwrap())
                .collect::<Vec<f32>>();

            let outputs = outputs
                .iter()
                .map(|t| t.parse().unwrap())
                .collect::<Vec<f32>>();

            cases.push((Vec::from(inputs), Vec::from(outputs)));
        }

        Ok((
            Params {
                seed: 5,
                memsize,
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
            memsize: 0,
            popsize: 100000,
            depth: 5,
            crossover_prob: 0.9,
            pmut_per_node: 0.05,
            tournament_size: 2,
            acceptable_error: -1e-3,
        }
    }
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "SEED={}
POPSIZE={}
DEPTH={}
CROSSOVER_PROB={}
PMUT_PER_NODE={}
TSIZE={}
----------------------------------\n",
                self.seed,
                self.popsize,
                self.depth,
                self.crossover_prob,
                self.pmut_per_node,
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
        let (_param, cases) = match Params::from_string(
            "2 | 3
2 2 | 4
1 3 | 4
10 20 | 30
"
                .to_owned(),
        ) {
            Ok(p) => p,
            Err(_) => panic!("Read problem failed"),
        };

        assert_eq!(cases.len(), 3);
        cases.iter().for_each(|(inputs, targets)| {
            assert_eq!(inputs.len(), 2);
            assert_eq!(targets.len(), 1);
        });
    }
}

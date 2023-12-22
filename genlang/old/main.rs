#[allow(dead_code, unused)]

const ABS: &str = "< R0 = x, output >
IF GT R0 0
  THEN
ELSE
  LOAD R0 MUL R0 -1
FI

RETURN R0";

type Num = f32;

struct Executor<'a> {
    cursor: usize,
    tokens: Vec<&'a str>,
    registers: Vec<Num>,
}

impl Executor<'_> {
    pub fn run(program: &str, registers: Vec<Num>) -> Num {
        let preprocessed = program.replace("\r", "").replace("\n", " ");
        let mut e = Executor {
            cursor: 0,
            registers,
            tokens: preprocessed.split(' ').filter(|x| x.len() > 0).collect(),
        };
        println!("{:?}", e.tokens);
        while e.cursor < e.tokens.len() {
            e.parse_block();
        }
        return e.registers[0];
    }

    fn skip_comment(&mut self) {
        while self.next() != ">" {}
        println!("{}", self.cursor);
    }

    fn next(&mut self) -> &str {
        self.cursor += 1;
        self.tokens[self.cursor]
    }

    fn parse_if(&mut self) {}

    fn parse_block(&mut self) {
        match self.next() {
            "<" => self.skip_comment(),
            "IF" => self.parse_if(),
            _ => {
                todo!();
            }
        }
    }
}

fn main() {
    println!("abs 2: {}", Executor::run(ABS, vec![2.0]));
    println!("abs -2: {}", Executor::run(ABS, vec![-2.0]));
}

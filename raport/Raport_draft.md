# Design

## [Grammar and tokens](../genlang/src/tinygp/common.rs)
```
# start token
block = stat*

# statement
stat
    = LOAD REG expr
    | IF expr block END
    | IF expr block ELSE block END
    | WHILE expr block END
    | INPUT REG
    | OUTPUT expr

# expression
expr
    = ADD expr expr
    | SUB expr expr
    | MUL expr expr
    | DIV expr expr
    | EQ expr expr
    | LT expr expr
    | GT expr expr
    | OR expr expr
    | AND expr expr
    | NOT expr
    | REG
    | NUM

# REG = register/memory cell
# NUM = a number literal
```

The grammar is designed to work with a single numeric type. Current implementation uses integers. For the sake of logical operators and condition checks (`IF` and `WHILE`) any non-zero number is treated as true and zero is treated as false.

## Parametrization
The project is heavily configurable, as almost every numeric parameter of the process is set inside [Params](../genlang/src/params.rs) structure.
This includes parameteres like population size, acceptable error, probabilities of mutation and crossover, and even probability distribution used in growing the program.

The program interpreter uses 32 bit integers as it's internal type, but can easily be adapted to floating point numbers with minimal development by assigning e.g. `type Number = f32;`, and adapting functions like `is_truthy`.

As of now, the fitness function is outside of the `Params` structure. However, because the fitting process can be run for a specific number of generations, and then be resumed after the program exits (thanks to serializing last generation), and the `evolve` method takes the fitness function as an argument, the fitness function may be changed mid-training with a change of a single line of code.

## Initial population
Useful programs will generally take some input and produce some output, therefore the initial programs are generated as `INPUT`, 2 random `STAT`, `OUTPUT`.

# Runnning
The program supports evolution as well as running programs from the console.

Reading text based problem files is not yet supported. Evolution has to be prepared as a benchmark in Rust code (see `src/benchmark`)

Benchmarks can then be run with
```
genlang -u <benchmark suite name> -g <generations> -s <seed>
```
Population is saved between executions of the program. Rerunning the command above will use the last saved generation as the starting one. In order to start fresh add `-f`.

A specific program with a specific input can be run with the following command. `program` is a lexpr. `inputs` is a space-separated list of inputs.
```
genlang -e <program> -i <inputs>
```

If this info got out of date, see `genlang --help`

# Logging
The program is able to produce numerous logs for diagnostics. Logging level is set via environment variable `RUST_LOG`. See https://docs.rs/env_logger/latest/env_logger/ to find out about advanced filtering.


# [Benchmarks](./Benchmarki.md)
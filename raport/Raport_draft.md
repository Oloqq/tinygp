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
    | WHILE expr DO block END
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
Useful programs will generally take some input and produce some output, therefore the initial programs are generated as `INPUT`, 2 random `STAT` , `OUTPUT`.

## Crossover

## Mutation


# Uruchamianie
[1_1_b_part_1](./1_1_b_part_1) pokazuje wynik po 1000 iteracji uruchomiony przez
`cargo run -- --suite 1_1_b -f -s 0 -g 1000`
TODO wyjasnic przelaczniki

pomiedzy uruchomieniami mozna dostosowac w [kodzie](../genlang/src/benchmark/bench_1_1.rs) parametry, zmienic przypadki testowe lub nawet funkcje dopasowania (TBD: ustalanie tego przez pliki konfiguracyjne)

nastepnie mozna kontynuowac uczenie na podstawie zapisanej populacji
`cargo run -- --suite 1_1_b -s 0 -g 100`

wynik w part_2
[1_1_b_part_2](./1_1_b_part_2)

mozna uruchomic istniejaca populacje bez modyfikowania przez podanie `-g 0`

# describe execution
`cargo run -- -e '((Stat . INPUT) (Reg . 0) (Stat . LOAD) (Reg . 4) (Expr Num . -74) (Stat . IF) (Expr . GT) (Expr Num . 69) (Reg . 1) (Stat . INPUT) (Reg . 0) ELSE (Stat . LOAD) (Reg . 2) (Reg . 2) END (Stat . OUTPUT) (Reg . 0))' -i '4 6'`


# describe logging

# Benchmarki testowe
Wyniki benchmarków można znaleźć w folderze [population](../genlang/population/)

W plikach `.pop` zapisano końcowe populacje. W plikach `.txt` można zobaczyć przebieg procesu uczenia oraz wymaganą ilość generacji.

- ✅Testy 1.1 (generowanie stałych)
  - sukces bez większych problemów
  - funkcja fitnesu `diff_first` oblicza różnicę miedzy oczekiwanym a otrzymanym pierwszym elementem z wyjścia. W przypadku pustego wyjścia przyjmuje 0 jako otrzymaną wartość.
- ✅Testy 1.2 (arytmetyka)
  - ważny jest dobór przypadków uczących. Jeśli liczby z wyjścia były bliskie oczekiwanemu wyjściu, programy osiadały na procesie przenoszenia wejścia na wyjście
  - użyto `diff_first`
- ✅Test 1.3 (zwracanie większej liczby)
  - w tych benchmarkach po raz pierwszy objawił się brak nacisku na tylko jeden element w outpucie
  - zmodyfikowano funkcję fitnesu, tak że wyliczony błąd osobnika mnożony był przez długość jego programu
  - w efekcie `diff_first` zaczęło promować puste wyjście
  - puste wyjście pozostało optymalne nawet przy dodaniu stałej jedynki do długości programu, przez przyjęcie 0 jako "domyślnej" wartości zwracanej przez program
  - po zmianie tej domyślnej wartości na $-\infty$ powstała funkcja `diff_first_promote_single`, zwracająca fitness
  $$-|actual-expected|\sqrt{len(program)}$$
- Test 1.4
  - 1.4.A (średnia z 10 liczb)
    - do generowania przypadków testowych użyto `troublemaker.py`
    - jako funkcję fitnesu znowu użyto `diff_first_promote_single`
    - nie dało efektu, prawdopodobnie problemem był wcześniejszy brak możliwości tworzenia nowych statementów poza krosowaniem
    - podczas mutacji statementu, dodano szansę na wygenerowanie w tym miejscu dodatkowego statementu zamiast zmieniania istniejącego. Szansa jest określana w parametrach jako `p_insertion`
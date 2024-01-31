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

# Benchmarki testowe
Wyniki benchmarków można znaleźć w folderze [population](../genlang/population/)

W plikach `.pop` zapisano końcowe populacje. W plikach `.txt` można zobaczyć przebieg procesu uczenia oraz wymaganą ilość generacji.

- ✅Testy 1.1 (generowanie stałych)
  - sukces bez większych problemów
  - funkcja fitnesu `diff_first` oblicza różnicę miedzy oczekiwanym a otrzymanym pierwszym elementem z wyjścia
- ✅Testy 1.2 (arytmetyka)
  - ważny jest dobór przypadków uczących. Jeśli liczby z wyjścia były bliskie oczekiwanemu wyjściu, programy osiadały na procesie przenoszenia wejścia na wyjście
  - użyto `diff_first`
- Test 1.3
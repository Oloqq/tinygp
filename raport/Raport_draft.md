[1_1_b_part_1](./1_1_b_part_1) pokazuje wynik po 1000 iteracji uruchomiony przez
`cargo run -- --suite 1_1_b -f -s 0 -g 1000`
TODO wyjasnic przelaczniki

pomiedzy uruchomieniami mozna dostosowac w [kodzie](../genlang/src/benchmark/bench_1_1.rs) parametry, zmienic przypadki testowe lub nawet funkcje dopasowania (TBD: ustalanie tego przez pliki konfiguracyjne)

nastepnie mozna kontynuowac uczenie na podstawie zapisanej populacji
`cargo run -- --suite 1_1_b -s 0 -g 100`

wynik w part_2
[1_1_b_part_2](./1_1_b_part_2)

(po wygenerowaniu tego zostały wprowadzone major zmiany)
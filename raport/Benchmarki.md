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
  - w późniejszym czasie zamieniono `-f32::INFINITY` na `f32::MIN`, ponieważ wystarczył jeden program o nieskończenie ujemnym fitnesie aby informacja o średniej sprawności była bezużyteczna.
- Test 1.4
  - 1.4.A (średnia z 10 liczb)
    - do generowania przypadków testowych użyto `troublemaker.py`
    - jako funkcję fitnesu znowu użyto `diff_first_promote_single`
    - nie dało efektu, prawdopodobnie problemem był wcześniejszy brak możliwości tworzenia nowych statementów poza krosowaniem
    - podczas mutacji statementu, dodano szansę na wygenerowanie w tym miejscu dodatkowego statementu zamiast zmieniania istniejącego. Szansa jest określana w parametrach jako `p_insertion`
    - udało się wygenerować program liczący średnią dla 3 liczb z fitnessem rzędu -3
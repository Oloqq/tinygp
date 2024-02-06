# Benchmarki testowe
Wyniki benchmarków można znaleźć w folderze [genlang/population](../genlang/population/)

W plikach `.pop` zapisano końcowe populacje. W plikach `.txt` można zobaczyć przebieg procesu uczenia oraz wymaganą ilość generacji.

Użyte funkcje fitness:
- `diff_first` - oblicza wartość bezwzględną różnicy miedzy oczekiwanym a otrzymanym pierwszym elementem z wyjścia, zwraca ją po przemnożeniu przz -1. W przypadku pustego wyjścia przyjmuje 0 jako otrzymaną wartość,
- `diff_best` - oblicza wartości bezwględne różnic miedzy oczekiwaną wartością a otrzymanymi elementami z wyjścia, wybiera najmniejszą z nich i zwraca ją po przemnożeniu przez -1,
- `diff_only` - oblicza różnicę miedzy oczekiwanym a otrzymanym pierwszym elementem z wyjścia, w przypadku większej liczby elementów na wyjściu, zwraca f32::MIN,
- `diff_first_promote_single` - działa jak `diff_first`, ale wyliczony błąd osobnika mnożony jeat przez pierwiastek z długości jego programu oraz przy braku elementów na wejściu zwracana jest wartość f32::MIN. W ten sposób promujemy programy zwracające mało elementów na wyjściu,
- `fit_arithmetic_series` - oblicza sumę róznic między odpowiadającymi sobie wartościami w ciągu oczekiwanym i zwracanym oraz dolicza "karę" (1000.0) w przypadku róznicy w długości ciągów,
- `fit-bool` - jeśli wartości logiczne na wyjściu się zgadzają zwraca 0, jeśli nie, -1

- ✅Testy 1.1 (generowanie stałych)
  - sukces bez większych problemów
  - W zależności od podpunktu użyta funkcja fitnesu to:
    - `diff_first`
    - `diff_best`
    - `diff_only`
- ✅Testy 1.2 (arytmetyka)
  - ważny jest dobór przypadków uczących. Jeśli liczby z wyjścia były bliskie oczekiwanemu wyjściu, programy osiadały na procesie przenoszenia wejścia na wyjście
  - użyto `diff_only` (jw.)
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

### Testy finalne
- Zadanie 3. (ciąg arytmetyczny)
  - Duże problemy z osiągnięciem dobrego wyniku
  - Funkcja fitness - `fit_arithmetic_series` - oblicza sumę róznic między odpowiadającymi sobie wartościami w ciągu oczekiwanym i zwracanym oraz dolicza "karę" w przypadku róznicy w długości ciągów
  - Parametry ewolucji:
    - Wielkość populacji: 5000
    - Maksymalny rozmiar programu: 200
    - Liczba pokoleń: 500
    - Liczba przypadków testowych: 50
  - Parametry ostatniego pokolenia:
    - Avg Fitness=-353218.4
    - Best Fitness=-338504
    - Avg Size=136

- Zadanie 17. (Suma kwadratów)
  - Suma fitness na poziomie -10 dla 10 przypadków testowych
  - Problem został podzielony na 2 podproblemy - obliczanie kwadratu liczby, oraz obliczanie sumy ciągu arytmetycznego (1 + n)/2
  - Jako funkcję fitnesu użyto `diff_first_promote_single` zarówno dla podproblemów, jak i dla zadania docelowego
  - Rozwiązywanie rozpoczęte od sumy populacji podproblemów
  - Parametry ewolucji zadania głównego:
    - Wielkość populacji: 5000
    - Maksymalny rozmiar programu: 200
    - Liczba pokoleń: 500
    - Liczba przypadków testowych: 50
  - Parametry ewolucji podproblemu 1 - sumy:
    - Wielkość populacji: 2000
    - Maksymalny rozmiar programu: 100
    - Liczba pokoleń: 500
    - Liczba przypadków testowych: 10
  - Parametry ewolucji podproblemu 2 - kwadrat liczby:
    - Wielkość populacji: 2000
    - Maksymalny rozmiar programu: 200
    - Liczba pokoleń: 10 (rozw. znalezione wcześniej)
    - Liczba przypadków testowych: 10
  - Parametry ostatniego pokolenia zadania głównego:
    - Avg Fitness=-inf
    - Best Fitness=-10
    - Avg Size=179

- Zadanie 28. (Minimum)
  - Suma fitness na poziomie -210 dla 100 przypadków testowych
  - Jako funkcję fitnesu użyto `diff_first_promote_single`
  - Parametry ewolucji:
    - Wielkość populacji: 5000
    - Maksymalny rozmiar programu: 200
    - Liczba pokoleń: 500
    - Liczba przypadków testowych: 100
  - Parametry ostatniego pokolenia zadania głównego:
    - Avg Fitness=-inf
    - Best Fitness=-210
    - Avg Size=173

- Regresja boolowska
  - Funkcja fitness - `fit_bool`
  - Dla niskich wartości k (1, 2), rozwiązanie jest znajdowane bez problemu
  - Dla wartości k 3 oraz 4 szybko znajdowane jest rozwiązanie zbliżone do oczekiwanego
  - Dla k = 5 oraz więcej, nie udaje się znaleźć rozwiązania
  - Tokeny języka ograniczono do przydatnych w tym problemie
  - Problemy mogą wynikać z nieprzystosowania systemu bezpośrednio do działania na zmiennych typu boolean (system interpretuje liczby całkowite jako wartości logiczne) i wykorzystanie języka mającego z założenia operować na liczbach całkowitych
  
  - Parametry ewolucji:
    - Wielkość populacji: 5000
    - Maksymalny rozmiar programu: 250
    - Liczba pokoleń: 200 lub 500
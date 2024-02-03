from random import randint
from math import floor
from itertools import product

# Boolean regression
def prob_bool(k: int):
    cases = product([0, 1], repeat=k)
    with open(f"problems/{k}", "w") as f:
        f.writelines(",\n".join(["(vec![" + ", ".join([str(val) for val in case]) + f"], vec![{randint(0, 1)}])" for case in cases]))

for i in range(1, 11):
    prob_bool(i)

# 1.4.A Program powinien odczytać dziesięć pierwszych liczy z wejścia i zwrócić na wyjściu (jedynie) ich średnią arytmetyczną (zaokrągloną do pełnej liczby całkowitej). Na wejściu mogą być tylko całkowite liczby w zakresie [-99,99]
def prob_1_4_a():
    inp = []
    for _ in range(3):
        inp.append(randint(-99, 99))

    a = sum(inp) / len(inp)

    case = f"(vec!{inp}, vec![{floor(a)}]),"
    print(case)

# for _ in range(10):
#     prob_1_4_a()


# 1.4.B Program powinien odczytać na początek z wejścia pierwszą liczbę (ma być to wartość nieujemna)
#  a następnie tyle liczb (całkowitych) jaka jest wartość pierwszej odczytanej liczby
#  i zwrócić na wyjściu (jedynie) ich średnią arytmetyczną zaokrągloną do pełnej liczby całkowitej
# (do średniej nie jest wliczana pierwsza odczytana liczba, która mówi z ilu liczb chcemy obliczyć średnią).
#  Na wejściu mogą być tylko całkowite liczby w zakresie [-99,99], pierwsza liczba może być tylko w zakresie [0,99].
# def prob_1_4_b():
    # inp = []
    # for _ in range(10):
    #     inp.append(randint(-99, 99))

    # a = sum(inp) / len(inp)

    # case = f"(vec!{inp}, vec![{floor(a)}]),"
    # print(case)
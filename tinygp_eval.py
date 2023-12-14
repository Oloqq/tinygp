import numpy as np
from sympy import simplify, lambdify, symbols, sympify


# def divide(numerator, denominator=0):
#     print("Success")
#     if abs(denominator) <= 0.001:
#         return numerator
#     else:
#         return numerator / denominator


def tinygp_eval(solution: str, X1: np.ndarray, X2: np.ndarray, simplify_flag: bool = False):
    print("TinyGP solution:", solution)
    x1_sym, x2_sym = symbols('X1'), symbols('X2')
    solution_sym = sympify(solution)
    if simplify_flag:
        simplified = simplify(solution_sym, ratio=1.7)
        print("Simplified solution:", simplified)
        return lambdify((x1_sym, x2_sym), simplified)(X1, X2)
    else:
        return lambdify((x1_sym, x2_sym), solution_sym)(X1, X2)

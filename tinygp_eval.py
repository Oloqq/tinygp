import numpy as np
from sympy import parsing, simplify, lambdify, symbols

def tinygp_eval(solution: str, X1: np.ndarray, X2: np.ndarray, simplify_flag: bool = False):
    # imports are used inside the eval function
    from numpy import sin, cos
    # TODO implement protected division - still necessary?
    print("TinyGP solution:", solution)
    if simplify_flag:
        simplified = simplify(parsing.sympy_parser.parse_expr(solution))
        print("Simplified solution:", simplified)
        x1_sym, x2_sym = symbols('X1'), symbols('X2')
        evaluated = lambdify((x1_sym, x2_sym), simplified, 'numpy')(X1, X2)
        return evaluated
    else:
        return eval(solution)

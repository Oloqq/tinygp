import matplotlib.pyplot as plt
import numpy as np
import make_problem
import inspect

SOLUTION_DIR = "output"
CHART_DIR = "charts"

def plot_chart(zadname: str):
    funcname = zadname[:4]
    assert funcname in ["zad1", "zad2", "zad3", "zad4", "zad5", "zad6"]
    original_func = getattr(make_problem, funcname)
    funcsrc = inspect.getsource(original_func)
    func_lines = funcsrc.split('\n')
    dims_by_func = func_lines[0].count(",") + 1
    formula = func_lines[1] \
        .partition('return ')[2] \
        .replace("**", "^") \
        .replace("*", "")

    with open(f"{SOLUTION_DIR}/{zadname}.dat") as f:
        lines = f.readlines()
        (dims, _consts, _minrand, _maxrand, case_num) = lines[0].split()
        dims = int(dims)
        domain_min = float(lines[1].split()[0])
        domain_max = float(lines[int(case_num)].split()[0])
        assert dims_by_func == dims
        # solved = lines[-1].strip() == "PROBLEM SOLVED"
        best_solution = lines[-3]

    print(formula)
    print(f"dimensions: {dims}")

    TASK = zadname
    FROM = domain_min
    TO = domain_max
    FUNC_STR = formula
    variables = []
    for _ in range(dims):
        variables.append(np.linspace(FROM, TO, 40))
    X1 = variables[0]
    X2 = variables[1]
    FUNC = original_func(*variables)
    TITLE = TASK + "\n" + f"{FUNC_STR}, [{FROM}, {TO}]"
    RESULT = eval(best_solution)

    def plot_one_dim():
        plt.scatter(X1, RESULT, label="Calculated result")
        plt.plot(X1, FUNC, color='red', label="Original function")
        plt.title(TITLE)
        plt.grid()
        plt.legend()
        plt.xlabel('x')
        plt.ylabel('f(x)')
        plt.savefig(f"{CHART_DIR}/{TASK}.png")


    def plot_two_dims():
        ax = plt.axes(projection='3d')
        ax.scatter3D(X1, X2, RESULT, label="Calculated result")
        ax.plot3D(X1, X2, FUNC, color='red', label="Original function")
        plt.xlabel('x')
        plt.ylabel('y')
        plt.title(TITLE)
        plt.grid()
        plt.legend()
        plt.savefig(f"{CHART_DIR}/{TASK}.png")

    if dims == 1:
        plot_one_dim()
    elif dims == 2:
        plot_two_dims()


import sys

if __name__ == "__main__":
    assert len(sys.argv) == 2
    plot_chart(sys.argv[1])

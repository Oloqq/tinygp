import sys
import matplotlib.pyplot as plt
import numpy as np
import make_problem
import inspect

SOLUTION_DIR = "output"
CHART_DIR = "charts"
DOMAINS = {
    "zad1a": (-10, 10),
    "zad1b": (0, 100),
    "zad1c": (-1, 1),
    "zad1d": (-1000, 1000),
    "zad2a": (-3.14, 3.14),
    "zad2b": (0, 7),
    "zad2c": (0, 100),
    "zad2d": (-100, 100),
    "zad3a": (0, 4),
    "zad3b": (0, 9),
    "zad3c": (0, 99),
    "zad3d": (0, 999),
    "zad4a": (0, 1),
    "zad4b": (-10, 10),
    "zad4c": (0, 100),
    "zad4d": (-1000, 1000),
    "zad5a": (-3.14, 3.14),
    "zad5b": (0, 7),
    "zad5c": (0, 100),
    "zad5d": (-100, 100),
    "zad7a": (-3.14, 3.14),
    "zad7b": (0, 7),
    "zad7c": (0, 100),
    "zad7d": (-100, 100),
    "zad6a": (-10, 10),
    "zad6b": (0, 100),
    "zad6c": (-1, 1),
    "zad6d": (-1000, 1000)
}


def plot_chart(zadname: str, resolution_arg: str = "80"):
    resolution = int(resolution_arg)
    funcname = zadname[:4]
    assert funcname in ["zad1", "zad2", "zad3", "zad4", "zad5", "zad6", "zad7"]
    original_func = getattr(make_problem, funcname)
    funcsrc = inspect.getsource(original_func)
    func_lines = funcsrc.split('\n')
    dims_by_func = func_lines[0].count(",") + 1
    formula = func_lines[1] \
        .partition('return ')[2] \
        .replace(" ** ", "^") \
        .replace("**", "^") \
        .replace(" * ", "") \
        .replace("*", "") \
        .replace("np.", "")

    with open(f"{SOLUTION_DIR}/{zadname}.dat") as f:
        lines = f.readlines()
        (dims, _consts, _minrand, _maxrand, case_num) = lines[0].split()
        dims = int(dims)
        assert dims_by_func == dims
        # solved = lines[-1].strip() == "PROBLEM SOLVED"
        best_solution = lines[-3]

    print(formula)
    print(f"dimensions: {dims}")

    domain_min = DOMAINS[zadname][0]
    domain_max = DOMAINS[zadname][1]
    variables = []
    for _ in range(dims):
        variables.append(np.linspace(domain_min, domain_max, resolution))
    X1 = variables[0]
    if dims > 1:
        X2 = variables[1]
        X1, X2 = np.meshgrid(X1, X2)
    func = original_func(*variables)
    title = zadname + "\n" + f"{formula}, [{domain_min}, {domain_max}]"
    result = eval(best_solution)

    def plot_one_dim():
        plt.scatter(X1, result, label="Calculated result")
        plt.plot(X1, func, color='red', label="Original function")
        plt.title(title)
        plt.grid()
        plt.legend()
        plt.xlabel('x')
        plt.ylabel('f(x)')
        plt.savefig(f"{CHART_DIR}/{zadname}.png")

    def plot_two_dims_scatter():
        z_result = eval(best_solution)
        z_original = original_func(X1, X2)
        fig = plt.figure()
        fig.set_size_inches(5, 6)
        ax1 = fig.add_subplot(1, 1, 1, projection='3d')

        step = 4
        x1_less_dense = decrease_density(X1, step)
        x2_less_dense = decrease_density(X2, step)
        z_res_less_dense = decrease_density(z_result, step)
        z_org_less_dense = decrease_density(z_original, step)

        ax1.scatter(x1_less_dense, x2_less_dense, z_res_less_dense, color='blue', s=2)
        ax1.scatter(x1_less_dense, x2_less_dense, z_org_less_dense, color='red', s=2)
        ax1.set_xlabel('x')
        ax1.set_ylabel('y')
        ax1.set_title(title + "\nCalculated result")
        ax1.grid()
        plt.savefig(f"{CHART_DIR}/{zadname}_scatter.png")

    def plot_two_dims():
        z_result = eval(best_solution)
        z_original = original_func(X1, X2)
        fig = plt.figure()
        fig.set_size_inches(12, 5)
        ax1 = fig.add_subplot(1, 2, 1, projection='3d')
        ax2 = fig.add_subplot(1, 2, 2, projection='3d')
        ax1.plot_surface(X1, X2, z_result)
        ax2.plot_surface(X1, X2, z_original, color='red')
        ax1.set_xlabel('x')
        ax2.set_xlabel('x')
        ax1.set_ylabel('y')
        ax2.set_ylabel('y')
        ax1.set_title(title + "\nCalculated result")
        ax2.set_title(title + "\nOriginal function")
        ax1.grid()
        ax2.grid()
        plt.savefig(f"{CHART_DIR}/{zadname}.png")

    if dims == 1:
        plot_one_dim()
    elif dims == 2:
        plot_two_dims()
        plot_two_dims_scatter()

def decrease_density(src: list, step: int) -> list:
    result = []
    for x in range(0, len(src), step):
        appendee = []
        for y in range(0, len(src[x]), step):
            appendee.append(src[x][y])
        result.append(appendee)
    return result

if __name__ == "__main__":
    assert len(sys.argv) in (2, 3)
    plot_chart(*sys.argv[1:])

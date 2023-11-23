import math
import numpy as np

# I didn't test this generic one

# def append_cases(dim, start, points, interval, sofar, foo, lines) -> list:
#     if dim == 1:
#         inputs = sofar
#         inputs.append(0)
#         for i in range(points):
#             inputs[-1] = start + interval * i
#             vars_serialized = " ".join(inputs)
#             lines.append(f"{vars_serialized} {foo(*inputs)}\n")
#         inputs.pop()
#     else:
#         for i in range(points):
#             append_cases(dim - 1, start, points, interval, sofar + [start + interval * i], foo, lines)


# def gen_problem(path, foo, domain, rand, points, dim=1):
#     header_line = f"{dim} {rand[0]} {rand[1]} {rand[2]} {points}\n"
#     lines = [header_line]
#     interval = (domain[1] - domain[0]) / points

#     append_cases(dim, domain[0], points, interval, [], foo, lines)

#     with open(path, "w") as f:
#         f.writelines(lines)

def gen_problem_1d(path, foo, domain, rand, points):
    lines = [f"1 {rand[0]} {rand[1]} {rand[2]} {points}\n"]

    interval = (domain[1] - domain[0]) / points
    for i in range(points):
        x = domain[0] + interval * i
        lines.append(f"{x} {foo(x)}\n")

    with open(path, "w") as f:
        f.writelines(lines)


def gen_problem_2d(path, foo, domain, rand, points):
    lines = [f"2 {rand[0]} {rand[1]} {rand[2]} {points}\n"]
    point_per_dim = math.ceil(math.sqrt(points))
    interval = (domain[1] - domain[0]) / point_per_dim

    for xi in range(point_per_dim):
        x = domain[0] + interval * xi
        for yi in range(point_per_dim):
            y = domain[0] + interval * yi
            lines.append(f"{x} {y} {foo(x, y)}\n")

    with open(path, "w") as f:
        f.writelines(lines)


def zad1(x):
    return 5 * x**3 - 2 * x**2 + 3 * x - 17


def zad2(x):
    return np.sin(x) + np.cos(x)


def zad3(x):
    return 2 * np.log(x + 1)


def zad4(x, y):
    return x + 2 * y


def zad5(x, y):
    return np.sin(x/2) + 2 * np.cos(y)
    # zakladam ze mialo byc cos(y)


def zad6(x, y):
    return x**2 + 3*x*y - 7*y + 1


def zad7(x, y):
    return np.sin(x/2) + 2 * np.cos(x)


def zad8(x):
    return np.sin(x + 3.141592/2)


def zad9(x):
    return np.tan(2*x + 1)


def main():
    rand = (100, -20, 20)

    # gen_problem_1d("zad1a.dat", zad1, (-10, 10), rand, 100)
    # gen_problem_1d("zad1b.dat", zad1, (0, 100), rand, 100)
    # gen_problem_1d("zad1c.dat", zad1, (-1, 1), rand, 50)
    # gen_problem_1d("zad1d.dat", zad1, (-1000, 1000), rand, 100)

    # gen_problem_1d("zad2a.dat", zad2, (-3.14, 3.14), rand, 100)
    # gen_problem_1d("zad2b.dat", zad2, (0, 7), rand, 100)
    # gen_problem_1d("zad2c.dat", zad2, (0, 100), rand, 100)
    # gen_problem_1d("zad2d.dat", zad2, (-100, 100), rand, 100)

    # gen_problem_1d("zad3a.dat", zad3, (0, 4), rand, 100)
    # gen_problem_1d("zad3b.dat", zad3, (0, 9), rand, 100)
    # gen_problem_1d("zad3c.dat", zad3, (0, 99), rand, 100)
    # gen_problem_1d("zad3d.dat", zad3, (0, 999), rand, 100)

    # gen_problem_2d("zad4a.dat", zad4, (0, 1), rand, 100)
    # gen_problem_2d("zad4b.dat", zad4, (-10, 10), rand, 100)
    # gen_problem_2d("zad4c.dat", zad4, (0, 100), rand, 100)
    # gen_problem_2d("zad4d.dat", zad4, (-1000, 1000), rand, 100)

    # gen_problem_2d("zad5a.dat", zad5, (-3.14, 3.14), rand, 100)
    # gen_problem_2d("zad5b.dat", zad5, (0, 7), rand, 100)
    # gen_problem_2d("zad5c.dat", zad5, (0, 100), rand, 100)
    # gen_problem_2d("zad5d.dat", zad5, (-100, 100), rand, 100)

    # gen_problem_2d("zad6a.dat", zad6, (-10, 10), rand, 100)
    # gen_problem_2d("zad6b.dat", zad6, (0, 100), rand, 100)
    # gen_problem_2d("zad6c.dat", zad6, (-1, 1), rand, 100)
    # gen_problem_2d("zad6d.dat", zad6, (-1000, 1000), rand, 100)

    # gen_problem_2d("zad7a.dat", zad7, (-3.14, 3.14), rand, 100)
    # gen_problem_2d("zad7b.dat", zad7, (0, 7), rand, 100)
    # gen_problem_2d("zad7c.dat", zad7, (0, 100), rand, 100)
    # gen_problem_2d("zad7d.dat", zad7, (-100, 100), rand, 100)

    gen_problem_1d("zad8.dat", zad8, (0, 2*np.pi), rand, 100)

    gen_problem_1d("zad9.dat", zad9, (-np.pi / 2, np.pi / 2), rand, 100)


if __name__ == "__main__":
    main()
    # rand = (20, -10, 10)
    # gen_problem_1d("zad2d.dat", zad2, (-100, 100), rand, 500)

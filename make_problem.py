import math

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
    interval = (domain[1] - domain[0]) / points

    for xi in range(points):
        x = domain[0] + interval * xi
        for yi in range(points):
            y = domain[0] + interval * yi
            lines.append(f"{x} {y} {foo(x, y)}\n")

    with open(path, "w") as f:
        f.writelines(lines)


def main():
    rand = (100, -20, 20)

    def zad1(x):
        return 5 * x**3 - 2 * x**2 + 3 * x - 17

    gen_problem_1d("zad1a.dat", zad1, (-10, 10), rand, 100)
    gen_problem_1d("zad1b.dat", zad1, (0, 100), rand, 100)
    gen_problem_1d("zad1c.dat", zad1, (-1, 1), rand, 50)
    gen_problem_1d("zad1d.dat", zad1, (-1000, 1000), rand, 100)

    def zad2(x):
        return math.sin(x) + math.cos(x)

    gen_problem_1d("zad2a.dat", zad2, (-3.14, 3.14), rand, 100)
    gen_problem_1d("zad2b.dat", zad2, (0, 7), rand, 100)
    gen_problem_1d("zad2c.dat", zad2, (0, 100), rand, 100)
    gen_problem_1d("zad2d.dat", zad2, (-100, 100), rand, 100)

    def zad3(x):
        return 2 * math.log(x + 1)

    gen_problem_1d("zad3a.dat", zad3, (0, 4), rand, 100)
    gen_problem_1d("zad3b.dat", zad3, (0, 9), rand, 100)
    gen_problem_1d("zad3c.dat", zad3, (0, 99), rand, 100)
    gen_problem_1d("zad3d.dat", zad3, (0, 999), rand, 100)

    def zad4(x, y):
        return x + 2 * y

    gen_problem_2d("zad4a.dat", zad4, (0, 1), rand, 100)
    gen_problem_2d("zad4b.dat", zad4, (-10, 10), rand, 100)
    gen_problem_2d("zad4c.dat", zad4, (0, 100), rand, 100)
    gen_problem_2d("zad4d.dat", zad4, (-1000, 1000), rand, 100)

    def zad5(x, y):
        # return math.sin(x/2) + 2 * math.cos(x) # chyba na upelu jest zadanie zapisane z bledem
        return math.sin(x/2) + 2 * math.cos(y) # zakladam ze mialo byc cos(y)

    gen_problem_2d("zad5a.dat", zad5, (-3.14, 3.14), rand, 100)
    gen_problem_2d("zad5b.dat", zad5, (0, 7), rand, 100)
    gen_problem_2d("zad5c.dat", zad5, (0, 100), rand, 100)
    gen_problem_2d("zad5d.dat", zad5, (-100, 100), rand, 100)

    def zad6(x, y):
        return x**2 + 3*x*y - 7*y + 1

    gen_problem_2d("zad6a.dat", zad6, (-10, 10), rand, 100)
    gen_problem_2d("zad6b.dat", zad6, (0, 100), rand, 100)
    gen_problem_2d("zad6c.dat", zad6, (-1, 1), rand, 100)
    gen_problem_2d("zad6d.dat", zad6, (-1000, 1000), rand, 100)


if __name__ == "__main__":
    main()

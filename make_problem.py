import math


def gen_problem(path, foo, domains, rand, points):
    variables = len(domains)
    lines = [f"{variables} {rand[0]} {rand[1]} {rand[2]} {points}\n"]

    interval = (domain[1] - domain[0]) / points
    for i in range(points):
        x = domain[0] + interval * i
        lines.append(f"{x} {foo(x)}\n")

    with open(path, "w") as f:
        f.writelines(lines)


def main():
    rand = (100, -20, 20)

    def zad1(x):
        return 5 * x**3 - 2 * x**2 + 3 * x - 17

    gen_problem("zad1a.dat", zad1, [(-10, 10)], rand, 100)
    gen_problem("zad1b.dat", zad1, [(0, 100)], rand, 100)
    gen_problem("zad1c.dat", zad1, [(-1, 1)], rand, 50)
    gen_problem("zad1d.dat", zad1, [(-1000, 1000)], rand, 100)

    def zad2(x):
        return math.sin(x) + math.cos(x)

    gen_problem("zad2a.dat", zad2, [(-3.14, 3.14)], rand, 100)
    gen_problem("zad2b.dat", zad2, [(0, 7)], rand, 100)
    gen_problem("zad2c.dat", zad2, [(0, 100)], rand, 100)
    gen_problem("zad2d.dat", zad2, [(-100, 100)], rand, 100)

    def zad3(x):
        return 2 * math.log(x + 1)

    gen_problem("zad3a.dat", zad3, [(0, 4)], rand, 100)
    gen_problem("zad3b.dat", zad3, [(0, 9)], rand, 100)
    gen_problem("zad3c.dat", zad3, [(0, 99)], rand, 100)
    gen_problem("zad3d.dat", zad3, [(0, 999)], rand, 100)

    def zad4(x, y):
        return x + 2 * y

    gen_problem("zad4a.dat", zad4, [(0, 1), (0, 1)], rand, 100)
    # TODO jak zagescic dane


if __name__ == "__main__":
    main()

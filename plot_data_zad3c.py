import matplotlib.pyplot as plt

TASK = "zad" + "3c"
N_GEN = 20
FROM = 0
TO = 99
FUNC_STR = "2 * log(x + 1)"
TITLE = TASK + "\n" + f"{FUNC_STR}, [{FROM}, {TO}]"
X1 = range(0, N_GEN + 1)
AVG_FIT = [
    169288480, 58399.484, 373.9315, 225.90172, 336.1946, 487.4094, 672.213, 817.6449, 983.4962, 1474.6869, 1284.9833, 1056.0743, 4366.7524, 3472.1794, 985302, 538517.7, 31802296, 64761.82, 28542.992, 5261.5913, 234639.3]
BEST_FIT = [27.11501, 27.11501, 22.282707, 22.282707, 13.023741, 13.023741, 12.923175, 11.909382, 11.3887005, 10.872756,
            9.485519, 9.485519, 8.603031, 8.603031, 7.4422603, 7.4422603, 7.4386744, 6.120191, 6.120191, 5.8710423, 5.162098]


def main():
    fig, (ax1, ax2) = plt.subplots(1, 2)
    fig.set_size_inches(12, 5)
    ax1.set_title(TITLE)
    ax2.set_title(TITLE)
    ax1.scatter(X1, AVG_FIT)
    ax2.scatter(X1, BEST_FIT)
    ax1.grid(), ax2.grid()
    ax1.set_xticks(range(21))
    ax2.set_xticks(range(21))
    ax1.set_xlabel('Number of Generation')
    ax2.set_xlabel('Number of Generation')
    ax1.set_ylabel("Average Fitness")
    ax2.set_ylabel("Best Fitness")
    plt.savefig(f"charts/{TASK}-data.png")


if __name__ == "__main__":
    main()

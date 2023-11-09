import matplotlib.pyplot as plt

TASK = "zad" + "6a"
N_GEN = 20
FROM = -10
TO = 10
FUNC_STR = "x^2 + 3xy - 7y + 1"
TITLE = TASK + "\n" + f"{FUNC_STR}, [{FROM}, {TO}]"
X1 = range(0, N_GEN + 1)
AVG_FIT = [15941570, 33153.035, 12800.407, 18264.17, 12843.575, 15596.292, 14360.267, 16812.793, 27256.967, 22602.111,
           18426.78, 59926.547, 32906.5, 139709.64, 22485.094, 16779.39, 85429.47, 88821090, 103164.93, 24517.857, 222395.38]
BEST_FIT = [5531.785, 5531.785, 5079.2656, 4823.933, 2858.6455, 2670.7493, 2631.52, 2387.358, 2146.9404, 1659.5497,
            948.0819, 779.6064, 739.3887, 701.1524, 144.75279, 144.75279, 144.75279, 142.12466, 84.67394, 80.40396, 67.870544]


def main():
    fig, (ax1, ax2) = plt.subplots(1, 2)
    fig.set_size_inches(12, 5)
    ax1.set_title(TITLE)
    ax2.set_title(TITLE)
    ax1.scatter(X1, AVG_FIT)
    ax2.scatter(X1, BEST_FIT)
    ax1.grid(), ax2.grid()
    ax1.set_xticks(range(N_GEN + 1))
    ax2.set_xticks(range(N_GEN + 1))
    ax1.set_xlabel('Number of Generation')
    ax2.set_xlabel('Number of Generation')
    ax1.set_ylabel("Average Fitness")
    ax2.set_ylabel("Best Fitness")
    plt.savefig(f"charts/{TASK}-data.png")


if __name__ == "__main__":
    main()

import matplotlib.pyplot as plt

TASK = "zad" + "1a"
N_GEN = 20
FROM = -10
TO = 10
FUNC_STR = "5x^3 - 2x^2 + 3x - 17"
TITLE = TASK + "\n" + f"{FUNC_STR}, [{FROM}, {TO}]"
X1 = range(0, N_GEN + 1)
AVG_FIT = [127819.2, 126820.42, 126082.016, 140600.14, 178524.83, 252293.78, 309485.44, 495402.28, 1709869.8, 19915720, 108793440, 12466168000,
           509628450, 4465420300, 1083177500000000, 736350200000, 1886478500000, 4884877300000000, 654488460000000, 6031183000000, 68159613000000]
BEST_FIT = [62982.07, 62975.54, 12734.82, 5596.669, 5430.368, 3396.79, 2239.4019, 1693.5471, 1301.5558, 682.53986,
            682.53986, 562.8546, 340.1105, 340.1105, 340.1105, 258.38416, 258.38416, 258.38416, 258.38416, 184.83675, 173.83235]


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

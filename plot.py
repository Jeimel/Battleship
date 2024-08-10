from argparse import ArgumentParser, ArgumentDefaultsHelpFormatter
from subprocess import run, PIPE
from os import name
from json import loads

import matplotlib.pyplot as plt
import matplotlib.ticker as mtick


def plot(data: list) -> None:
    data = [y / 100_000 for y in data]

    plt.plot(data, linewidth="4")

    plt.title("Simulation of Standard Battleship Game", loc="left")
    plt.xlabel("Number of Shots")
    plt.ylabel("Games Won")
    plt.xlim(0, 99)
    plt.grid(axis="y")

    plt.gca().yaxis.set_major_formatter(mtick.PercentFormatter(xmax=1.0))

    plt.show()


def main() -> None:
    parser = ArgumentParser(
        description="Battleship: Run a simulation.",
        formatter_class=ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "-n",
        type=int,
        help="Number of simulations to run.",
        default=10_000,
    )

    args = parser.parse_args()

    commands = ["cargo", "build", "--release", "--quiet"]
    result = run(commands)
    if result.returncode != 0:
        exit(1)

    exe_path = "target/release/battleship"
    if name == "nt":
        exe_path += ".exe"

    commands = [exe_path, str(args.n)]

    result = run(commands, stdout=PIPE, stderr=PIPE, universal_newlines=True)
    if result.returncode != 0:
        exit(1)

    data = loads(result.stdout)
    plot(data)


if __name__ == "__main__":
    main()

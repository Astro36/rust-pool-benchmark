from argparse import ArgumentParser
import csv
from itertools import groupby
import matplotlib.pyplot as plt
import matplotlib as mpl


def time(x, pos):
    if x >= 1e6:
        return f'{x / 1e6}ms'
    else:
        return f'{x / 1e3}µs'


def keyfn(row):
    return (int(row[1]), int(row[2]))


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument('path', help='benchmark result')
    args = parser.parse_args()

    mpl.rcParams['figure.autolayout'] = True

    with open(args.path) as file:
        reader = csv.reader(file)
        table = [[row[0], int(row[1]), int(row[2]), int(row[3]), int(row[4])] for row in reader]
        table.sort(key=keyfn)
        for key, group in groupby(table, keyfn):
            group = list(group)
            labels = [row[0] for row in group]
            values = [row[3] for row in group]

            fig, (ax1, ax2) = plt.subplots(1, 2, sharey=True)
            fig.subplots_adjust(hspace=0.05)

            ax1.barh(labels, values)
            ax2.barh(labels, values)

            sorted_values = sorted(values)
            ax1.set_xlim(0, sorted_values[4] * 1.2)
            ax2.set_xlim(sorted_values[5] * 0.95, sorted_values[-1] * 1.05)
            ax1.xaxis.set_major_formatter(time)
            ax2.xaxis.set_major_formatter(time)

            fig.savefig(f'p{key[0]:02}_w{key[1]:02}.svg')

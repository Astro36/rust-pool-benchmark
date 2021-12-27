from argparse import ArgumentParser
import csv
from itertools import groupby
import matplotlib.pyplot as plt
import matplotlib as mpl


def benchmark_input(row):
    return (row[1], row[2])


def format_time(x, pos):
    if x >= 1e6:
        return f'{x / 1e6:.1f}ms'
    else:
        return f'{x / 1e3:3.0f}µs'


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument('path', help='benchmark result')
    args = parser.parse_args()

    plt.style.use('seaborn-notebook')
    mpl.rcParams['axes.edgecolor'] = '#676466'
    mpl.rcParams['axes.facecolor'] = '#f5f4f3'
    mpl.rcParams['axes.prop_cycle'] = "cycler('color', ['#6768ab'])"
    mpl.rcParams['figure.autolayout'] = True
    mpl.rcParams['ytick.color'] = '#676466'

    with open(args.path) as file:
        reader = csv.reader(file)
        table = [[row[0], int(row[1]), int(row[2]), int(row[3]), int(row[4])] for row in reader]
        table.sort(key=benchmark_input)
        for key, group in groupby(table, benchmark_input):
            group = list(group)
            group.sort(key=lambda row: row[0], reverse=True)

            labels = [row[0] for row in group]
            values = [row[3] for row in group]
            formatted_values = [format_time(value, 0) for value in values]
            sorted_values = sorted(values)
            margin = sorted_values[-1] / 18

            fig, (ax1, ax2) = plt.subplots(1, 2, sharey=True)
            fig.subplots_adjust(wspace=0.05)
            fig.suptitle(f'Benchmark (pool={key[0]}, workers={key[1]})')
            bar1 = ax1.barh(labels, values)
            bar2 = ax2.barh(labels, values)
            ax1.bar_label(bar1, labels=formatted_values, padding=-35, color='white')
            ax2.bar_label(bar2, labels=formatted_values, padding=3, color='#676466')
            ax1.set_xlim(0, sorted_values[4] * 1.1)
            ax2.set_xlim(sorted_values[5] - margin, sorted_values[-1] + margin)
            ax1.spines.right.set_visible(False)
            ax2.spines.left.set_visible(False)
            ax1.xaxis.set_major_formatter(format_time)
            ax2.xaxis.set_major_formatter(format_time)
            ax1.xaxis.set_visible(False)
            ax2.xaxis.set_visible(False)
            ax2.yaxis.set_ticks_position('none')
            fig.savefig(f'benchmark(p{key[0]:02}_w{key[1]:02}).svg')

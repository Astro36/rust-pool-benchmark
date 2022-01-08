import csv
from itertools import groupby
import matplotlib.pyplot as plt
import matplotlib as mpl


def benchmark_input(row):
    return (row[1], row[2])


def format_time(x, pos):
    return f' {int(x / 1e6):,}ms '


if __name__ == "__main__":
    mpl.rcParams['axes.edgecolor'] = '#676466'
    mpl.rcParams['axes.facecolor'] = '#f5f4f3'
    mpl.rcParams['axes.prop_cycle'] = "cycler('color', ['#6768ab'])"
    mpl.rcParams['figure.autolayout'] = True
    mpl.rcParams['figure.titlesize'] = 16
    mpl.rcParams['font.family'] = 'monospace'
    mpl.rcParams['font.size'] = 9
    mpl.rcParams['text.color'] = '#2d282e'
    mpl.rcParams['ytick.labelcolor'] = '#2d282e'
    mpl.rcParams['ytick.labelsize'] = 10
    mpl.rcParams['ytick.color'] = '#676466'

    with open('benchmark.txt') as file:
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
            margin = sorted_values[-1] / 10

            fig, (ax1, ax2) = plt.subplots(1, 2, sharey=True)
            fig.subplots_adjust(wspace=0.05)
            fig.suptitle(f'Benchmark (pool={key[0]}, workers={key[1]})')
            bar1 = ax1.barh(labels, values)
            bar2 = ax2.barh(labels, values)
            label1 = ax1.bar_label(bar1, labels=formatted_values, color='white')
            label2 = ax2.bar_label(bar2, labels=formatted_values)

            for label in label1:
                label.set_horizontalalignment('right')

            ax1.set_xlim(0, sorted_values[3] * 1.1)
            ax2.set_xlim(sorted_values[4] - margin, sorted_values[5] + margin)
            ax1.spines.right.set_visible(False)
            ax2.spines.left.set_visible(False)
            ax1.xaxis.set_major_formatter(format_time)
            ax2.xaxis.set_major_formatter(format_time)
            ax1.xaxis.set_visible(False)
            ax2.xaxis.set_visible(False)
            ax2.yaxis.set_ticks_position('none')

            fig.savefig(f'benchmark(p{key[0]:02}_w{key[1]:02}).svg')

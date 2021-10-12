import argparse
import csv
import contextlib
import math
import os
import sys

from matplotlib import pyplot as plt
import numpy as np


@contextlib.contextmanager
def new_figure(filename):
    fig = plt.figure()
    try:
        yield fig
    finally:
        fig.tight_layout()
        fig.savefig(filename, dpi=500)
        print(f'Saved fig {filename}')


def parse_args():
    parser = argparse.ArgumentParser(description="Plot Summary")
    parser.add_argument('filenames', nargs='+',
                        help='csv files to be included in summary')
    parser.add_argument('-d', '--output-dir',
                        help='where to put the plot files', default='.')

    return parser.parse_args()


def csv_to_dict(filename):
    """
    return a csv reader
    """
    with open(filename, 'r') as fp:
        reader = csv.DictReader(fp)
        return {row.get('name'): row for row in reader}


def simplify_name(name):
    return os.path.basename(name)


def simplify_names(names):
    return list(map(simplify_name, names))


def get_column_by_file(filesdata, column):
    """
    Return a dict containing the specified column by file
    """
    ret = {}
    for filename, data in filesdata.items():
        ret[filename] = {key: row.get(column) for key, row in data.items()}

    return ret


def plot_compare_all(ax, instance_names, filesdata):
    """
    Plot total cost of all files of all instances.
    """
    total_costs = get_column_by_file(filesdata, 'solution_value')
    for filename, data in total_costs.items():
        values = [float(data.get(name, 'nan')) for name in instance_names]
        ax.plot(
            simplify_names(instance_names),
            values,
            label=filename
        )

    if len(filesdata.keys()) > 7:
        legend_params = dict(bbox_to_anchor=(0, 1.02, 1, .3), ncol=2)
    else:
        legend_params = dict(loc='upper left')

    ax.legend(fontsize=8, **legend_params)
    ax.autoscale()
    ax.set_xlabel('Instance name')
    ax.set_ylabel('Total cost')
    ax.tick_params(axis='x', labelrotation=90)


def plot_total_cost_histogram(ax, instance_names, filesdata):
    """
    Plot histogram of the sum of total cost
    """
    total_costs = get_column_by_file(filesdata, 'solution_value')
    distances = get_column_by_file(filesdata, 'distance')

    labels = list(filesdata.keys())
    locs = np.arange(len(labels))
    width = 0.35  # the width of the bars

    agg_data = []
    for label in labels:
        agg_data.append(dict(
            label=label,
            total_cost_sum=sum(map(float, total_costs[label].values())),
            distance_sum=sum(map(float, distances[label].values())),
        ))

    agg_data.sort(key=lambda d: d.get('total_cost_sum'))
    labels = [d.get('label') for d in agg_data]
    total_costs_values = [d.get('total_cost_sum') for d in agg_data]
    distances_values = [d.get('distance_sum') for d in agg_data]

    ax.bar(locs - width / 2, total_costs_values, width, label='Total Cost Sum')
    ax.bar(locs + width / 2, distances_values, width, label='Distance Sum')

    ax.autoscale()
    ax.legend()
    ax.set_xlabel('File name')
    ax.set_ylabel('Total cost')
    ax.set_xticks(locs)
    ax.set_xticklabels(list(map(simplify_name, labels)))
    ax.tick_params(axis='x', labelrotation=90)


def plot_files_data(output_dir, instance_names, filesdata):
    """
    Make different plots for the given instances for each filedata
    """
    compare_all_filename = os.path.join(output_dir, 'compare_all.png')
    with new_figure(compare_all_filename) as fig:
        ax1 = fig.add_subplot()
        plot_compare_all(ax1, instance_names, filesdata)

    total_cost_histogram = os.path.join(output_dir, 'total_cost_histogram.png')
    with new_figure(total_cost_histogram) as fig:
        ax1 = fig.add_subplot()
        plot_total_cost_histogram(ax1, instance_names, filesdata)


def main():
    """
    Creates a plot from several summary csvs
    """
    args = parse_args()
    instance_names = set()
    filedata = {}

    for filename in args.filenames:
        data = csv_to_dict(filename)
        instance_names = instance_names | set(data.keys())
        filedata[filename] = data

    instance_names = list(sorted(instance_names))
    plot_files_data(args.output_dir, instance_names, filedata)


if __name__ == '__main__':
    main()

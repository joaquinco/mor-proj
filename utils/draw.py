from __future__ import print_function

import json
import os
import sys

from matplotlib import pyplot as plt, colors as mpl_colors

from misc import log_err


def not_white(value):
    if not isinstance(value, str):
        return False

    lvalue = value.lower()
    return not(lvalue == 'white' or lvalue.startswith('#f'))


def get_colors():
    return list(filter(
        not_white,
        mpl_colors.get_named_colors_mapping().values()
    ))


COLORS = get_colors()
SOURCE_COLOR = 'red'
CLIENTS_COLOR = 'gray'


def draw_route(ax, clients, linewidth, color):
    """
    Performs actual drawing
    """
    client_pairs = zip(clients[:-1], clients[1:])

    for c1, c2 in client_pairs:
        p1 = c1.get('pos')
        p2 = c2.get('pos')
        x, y = list(zip(p1, p2))
        ax.plot(x, y, linewidth=linewidth, color=color)


def draw_solution_to_figure(data):
    """
    Draw a solution into a matplotlib Figure
    """
    solution = data['solution']
    instance = data['instance']

    vehicles_by_id = {v.get('id'): v for v in instance.get('vehicles')}
    clients_by_id = {c.get('id'): c for c in instance.get('clients')}
    routes = solution.get('routes')

    fig = plt.figure()
    ax1 = fig.add_subplot()

    colors = COLORS[:]

    for route in routes:
        color = colors.pop()
        clients = [clients_by_id[c.get('client_id')]
                   for c in route.get('clients')]

        vehicle_fixed_cost = vehicles_by_id[route.get(
            'vehicle_id')].get('fixed_cost')
        draw_route(ax1, clients, 2, color)

    for client in clients_by_id.values():
        point = client.get('pos')
        ax1.scatter(*point, marker='o', color=CLIENTS_COLOR)
        ax1.annotate(client.get('id'), point)

    ax1.scatter(*clients_by_id.get(instance.get('source')).get('pos'),
                marker='s', linewidth=4, color=SOURCE_COLOR)

    return fig


def draw_solution(filename):
    """
    Open solution file, call draw and saves the figure.
    """
    file_base, _ = os.path.splitext(filename)
    output = "{}.png".format(file_base)

    with open(filename, "r") as file:
        data = json.loads(file.read())

        fig = draw_solution_to_figure(data)
        fig.savefig(output, dpi=300)

        print(output)


def main(args):
    if not args:
        log_err("You must specify at least one file")
        sys.exit(1)

    for filename in args:
        draw_solution(filename)


if __name__ == '__main__':
    main(sys.argv[1:])

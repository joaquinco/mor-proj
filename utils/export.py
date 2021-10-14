from __future__ import print_function

import argparse
import json
import math
import os
import re
import sys

from misc import log_err


class InfoNotFoundError(Exception):
    pass


def get_array_content(array_string):
    return array_string.split('[')[1].split(']')[0].rstrip(';')


def as_int_array(value):
    return list(map(int, value.split()))


def euclidean(p1, p2):
    return math.sqrt(sum([
        (p1[i] - p2[i]) ** 2 for i in range(0, len(p1))
    ]))


def get_clients(node_infos: str):
    clients = []

    for index, node_info in enumerate(node_infos):
        parts = list(map(float, node_info.rstrip(';').split()))
        clients.append({
            "demand": int(parts[3]),
            "earliest": int(parts[4]),
            "latest": int(parts[5]),
            "service_time": int(parts[6]),
            "pos": [parts[1], parts[2]],
        })

    return clients


def get_distances(clients):
    distances = [[0] * len(clients) for _ in clients]

    for idx1, c1 in enumerate(clients):
        for idx2, c2 in enumerate(clients):
            distances[idx1][idx2] = euclidean(c1.get('pos'), c2.get('pos'))

    return distances


graph_regex = re.compile(
    'datos\s*=\s*\[(((\s*\d+\s*){7};)*(\s*\d*\s*){7})',
    re.MULTILINE,
)


def extract_graph(content):
    """
    Generate node and distance matrix from something like:
    nodes = [1 2 3 4 8 9;
             2 3 4 5 1 23;
             3 6 7 8 4 5]
    Where the columns have the following meaning:
      1. id
      2. position x
      3. position y
      4. demand
      5. earliest time
      6. latest time
      7. service time
    """
    match = graph_regex.search(content)
    if not match:
        raise InfoNotFoundError("Missing nodes information (datos=[...];")

    node_matches = list(filter(
        bool, map(lambda e: e.strip(), get_array_content(match.group(0)).split(';'))))

    clients = get_clients(node_matches)
    distances = get_distances(clients)

    return clients, distances


def array_regex(var_name):
    return re.compile('\\b{}\s*=\s*\[((\s*\d+\s*)+)\];'.format(var_name))


capacity_regex = array_regex('q')
fixed_cost_regex = array_regex('f')
variable_cost_regex = array_regex('alpha')
vehicle_type_regex = re.compile(
    'SC\s*=\s*\[(((\s*\d+\s*)+;)*(\s*\d+\s*))+\];',
    re.MULTILINE
)


def extract_vehicle_definitions(content):
    """
    Obtain vehicle definitions in the form:
      q=[200 100];
      f=[80 40];
      alpha=[1 1];
      SC=[1 1 1 0; 0 0 0 1;];

    Which will result in:
    [{
      capacity: 200,
      fixed_cost: 80,
      variable_cost: 1,
    }]
    """
    values = dict(
        capacities=capacity_regex,
        fixed_costs=fixed_cost_regex,
        variable_costs=variable_cost_regex,
        vehicle_types=vehicle_type_regex,
    )

    for key, regex in list(values.items()):
        match = regex.search(content)

        if not match:
            raise InfoNotFoundError(
                'Missing  vehicles information: {}'.format(key))

        values[key] = match.group(0)

    ret = []

    capacities = as_int_array(get_array_content(values['capacities']))
    fixed_costs = as_int_array(get_array_content(values['fixed_costs']))
    variable_costs = as_int_array(get_array_content(values['variable_costs']))
    vehicle_types = get_array_content(values['vehicle_types'])

    try:
        for index, type_array in enumerate(vehicle_types.split(';')):
            vehicle_count = sum(map(int, type_array.split()))

            ret.append({
                "count": vehicle_count + 2,
                "capacity": capacities[index],
                "fixed_cost": fixed_costs[index],
                "variable_cost": variable_costs[index],
            })
    except IndexError:
        raise InfoNotFoundError(
            'Vehicle information is malformed\nq={}\nf={}\nalpha={}\n'.format(
                capacities, fixed_costs, variable_costs
            )
        )

    return ret


def export_file_to_config(filename, allowed_deviation, deviation_penalty, suffix=''):
    """
    Generates a configuration file from a CIPLEX .m source
    """
    file_base, _ = os.path.splitext(filename)
    if suffix:
        suffix = '_' + suffix
    file_base = "{}{}".format(file_base, suffix)
    outputname = file_base + ".json"

    with open(filename, 'r') as file:
        # File content without comments
        content = "\n".join(
            filter(lambda line: not line.startswith('%'), list(file)))

        clients, distances = extract_graph(content)
        vehicle_definitions = extract_vehicle_definitions(content)

        with open(outputname, 'w') as out:
            out.write(json.dumps({
                "name": file_base,
                "distances": distances,
                "clients": clients,
                "vehicle_definitions": vehicle_definitions,
                "allowed_deviation": allowed_deviation,
                "deviation_penalty": deviation_penalty,
            }))

        print(outputname)


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument('filenames', help='Matlab files to convert', nargs='+')
    parser.add_argument('--allowed-deviation', type=float, default=0.5)
    parser.add_argument('--deviation-penalty', type=float, default=0.1)
    parser.add_argument('--suffix', default='')

    args = parser.parse_args(argv)

    for filename in args.filenames:
        try:
            export_file_to_config(
                filename,
                args.allowed_deviation,
                args.deviation_penalty,
                suffix=args.suffix,
            )
        except InfoNotFoundError as e:
            log_err("Error in file {}: {}".format(filename, e))


if __name__ == '__main__':
    main(sys.argv[1:])

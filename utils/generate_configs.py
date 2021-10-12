import copy
import json
import os
import sys

from itertools import combinations

value_range = [0.0, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9, 1]

distance_weights = value_range
time_weights = value_range
wait_time_weights = [0.0, 0.1, 0.2, 0.3, 0.5]


def almost_equal(v1, v2):
    return abs(abs(v1) - abs(v2)) < 1e-7


def _do_normalized_combinations(value_left, current, *iters):
    """
    Recurively iterates and return tuples
    """
    if len(iters) == 0:
        if almost_equal(value_left, 0):
            yield current
    else:
        for value in iters[0]:
            yield from _do_normalized_combinations(value_left - value, current + [value], *iters[1:])


def normalized_combinations(*iters, normalized_to=1):
    """
    Return normalized tuples of numbers
    """
    return _do_normalized_combinations(normalized_to, [], *iters)


def generate_config_files(filename, base_config):
    """
    Generate multiple configurations files based on base_config
    """
    value_combinations = normalized_combinations(
        distance_weights,
        time_weights,
        wait_time_weights
    )

    for index, values in enumerate(value_combinations):
        d, t, w = values

        new_config = copy.deepcopy(base_config)
        new_config['grasp_config'].update(
            distance_weight=d,
            time_weight=t,
            wait_time_weight=w,
        )

        with open(f'{filename}_{index}.json', 'w') as fp:
            fp.write(json.dumps(new_config))


def main(filename, *args):
    with open(filename, 'r') as f:
        file_without_ext, _ = os.path.splitext(filename)
        generate_config_files(file_without_ext, json.loads(f.read()))


if __name__ == '__main__':
    main(*sys.argv[1:])

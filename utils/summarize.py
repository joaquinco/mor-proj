import json
import sys

from misc import log_err


def print_line(values, separator=','):
    print(separator.join(map(str, values)))


def print_all(data):
    """
    Print csv like that to stdout
    """
    if not data:
        return

    header = list(data[0].keys())
    print_line(header)
    for entry in data:
        print_line([
            entry.get(key, '') or '' for key in header
        ])


def get_max_wait_time(solution):
    """
    Return maximun wait time of all route entries.
    """
    return max([
        max(*[c.get('wait_time') for c in route.get('clients')])
        for route in solution.get('routes')
    ])


def process_solution(data):
    """
    Given a solution file, returns a dictionary with the main data.
    """
    name = data.get('name')
    solution = data.get('solution')
    instance = data.get('instance')

    return dict(
        name=name,
        penalty=instance.get('deviation_penalty'),
        allowed_deviation=instance.get('allowed_deviation'),
        distance=solution.get('distance'),
        solution_value=solution.get('value'),
        number_of_vehicles=len(solution.get('routes')),
        max_wait_time=get_max_wait_time(solution),
        iter_found=solution.get('iter_found'),
    )


def main(files):
    """
    Print instance name and solution values to stdout
    """
    data = []
    for filename in files:
        try:
            with open(filename, "r") as f:
                content = json.loads(f.read())
            data.append(process_solution(content))
        except Exception as e:
            log_err(f'Couldn\'t process file {filename} because: {e}\n')

    print_all(data)


if __name__ == '__main__':
    main(sys.argv[1:])

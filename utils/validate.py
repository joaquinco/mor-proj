import json
import sys

from misc import log_err


def validate_solution(data):
  instance = data.get('instance')
  distances = instance.get('distances')
  vehicles = {v.get('id'): v for v in instance.get('vehicles')}
  solution = data.get('solution')
  routes = solution.get('routes')

  val = 0
  for route in routes:
    clients = route.get('clients')
    val += vehicles.get(route.get('vehicle_id')).get('fixed_cost')
    for c1, c2 in zip(clients[1:], clients[:-1]):
      val += distances[c1.get('client_id')][c2.get('client_id')]

  solution_val = solution.get('value')
  assert val == solution_val, f"Expected solution value of {val}, found {solution_val}"


def validate_solution_from_file(filename):
  with open(filename, 'r') as f:
    data = json.loads(f.read())
    validate_solution(data)


def main(args):
  if not args:
    log_err("You must specify at least one file")
    sys.exit(1)

  for filename in args:
    validate_solution_from_file(filename)


if __name__ == '__main__':
  main(sys.argv[1:])

import json
import sys

from misc import log_err


def validate_solution(data):
  instance = data.get('instance')
  distances = instance.get('distances')
  clients = {c.get('id'): c for c in instance.get('clients')}
  vehicles = {v.get('id'): v for v in instance.get('vehicles')}
  solution = data.get('solution')
  routes = solution.get('routes')

  allowed_deviation = instance.get('allowed_deviation')

  val = 0
  for route in routes:
    route_clients = route.get('clients')
    val += vehicles.get(route.get('vehicle_id')).get('fixed_cost')
    current_time = 0
    for c1, c2 in zip(route_clients[1:], route_clients[:-1]):
      client2 = clients.get(c2.get('client_id'))
      arc_time = distances[c1.get('client_id')][c2.get('client_id')]
      current_time = max(client2.get('earliest'), arc_time + current_time)
      allowed_offset = (client2.get('latest') - client2.get('earliest')) * allowed_deviation
      client_latest = client2.get('latest') + allowed_deviation

      assert current_time <= client_latest, f"Client {client2.get('id')} arrival time is {current_time} but latest is {client_latest}"

      current_time += client2.get('service_time')
      val += arc_time

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

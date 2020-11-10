import json
import sys

from misc import log_err

class Errors:
  def __init__(self, name):
    self.messages = []
    self.name = name

  def assert_cond(self, condition, message):
    if not condition:
      self.messages.append(message)

  def __bool__(self):
    return bool(self.messages)

  def print(self):
    lines = [f'Errors for {self.name}:']
    for m in self.messages:
      if isinstance(m, Errors):
        if m:
          lines.append(m.print())
      else:
        lines.append(f'- {m}')

    return '\n'.join(lines)


def validate_solution(data):
  instance = data.get('instance')
  distances = instance.get('distances')
  clients = {c.get('id'): c for c in instance.get('clients')}
  vehicles = {v.get('id'): v for v in instance.get('vehicles')}
  solution = data.get('solution')
  routes = solution.get('routes')
  allowed_deviation = instance.get('allowed_deviation')

  errors = Errors(data.get('name'))

  val = 0
  for route in routes:
    route_clients = route.get('clients')
    vehicle_id = route.get('vehicle_id')
    vehicle = vehicles.get(vehicle_id)
    val += vehicle.get('fixed_cost')
    capacity_left = vehicle.get('capacity')
    current_time = 0

    route_errors = Errors(f'Vehicle {vehicle_id}')
    for c1, c2 in zip(route_clients[1:], route_clients[:-1]):
      client2 = clients.get(c2.get('client_id'))
      arc_time = distances[c1.get('client_id')][c2.get('client_id')]
      current_time = max(client2.get('earliest'), arc_time + current_time)
      allowed_offset = (client2.get('latest') - client2.get('earliest')) * allowed_deviation
      client_latest = client2.get('latest') + allowed_deviation
      capacity_left -= client2.get('demand')
      arrive_time = c2.get('arrive_time')

      route_errors.assert_cond(
        arrive_time == current_time,
        f"Exected arrival time to {client2.get('id')} was {current_time} but found {arrive_time}"
      )
      route_errors.assert_cond(
        current_time <= client_latest,
        f"Client {client2.get('id')} arrival time is {current_time} but latest is {client_latest}"
      )

      current_time += client2.get('service_time')
      val += arc_time

    route_errors.assert_cond(capacity_left >= 0, f'Capacity overpassed by {-capacity_left} on vehicle {vehicle_id}')
    errors.assert_cond(not route_errors, route_errors)

  solution_val = solution.get('value')
  errors.assert_cond(val == solution_val, f"Expected solution value of {val}, found {solution_val}")

  return errors


def validate_solution_from_file(filename):
  with open(filename, 'r') as f:
    data = json.loads(f.read())
    errors = validate_solution(data)
    if errors:
      log_err(f'{errors.print()}\n')


def main(args):
  if not args:
    log_err("You must specify at least one file")
    sys.exit(1)

  for filename in args:
    validate_solution_from_file(filename)


if __name__ == '__main__':
  main(sys.argv[1:])

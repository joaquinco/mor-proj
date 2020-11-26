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
      entry.get(key,'') or '' for key in header
    ])


def process_solution(data):
  """
  Given a solution file, returns a dictionary with the main data.
  """
  name = data.get('name')
  solution = data.get('solution')

  return dict(
    name=name,
    distance=solution.get('distance'),
    solution_value=solution.get('value'),
    number_of_vehicles=len(solution.get('routes')),
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

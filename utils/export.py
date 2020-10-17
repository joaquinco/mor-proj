from __future__ import print_function

import json
import math
import os
import re
import sys


class InfoNotFoundError(Exception):
  pass


def log_err(*args, **kwargs):
  sys.stderr.write(*args, **kwargs)


graph_regex = re.compile('((\d+\s*){7};)')

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


def euclidean(p1, p2):
  return math.sqrt(sum([
    (p1[i] - p2[i]) ** 2 for i in range(0, len(p1))
  ]))


def get_distances(clients):
  distances = [[0] * len(clients) for _ in clients]

  for idx1, c1 in enumerate(clients):
    for idx2, c2 in enumerate(clients):
      distances[idx1][idx2] = int(euclidean(c1.get('pos'), c2.get('pos')))

  return distances


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
  node_matches = [match for match, _ in graph_regex.findall(content)]

  if not node_matches:
    raise InfoNotFoundError("Missing nodes information")

  clients = get_clients(node_matches)
  distances = get_distances(clients)
  
  return clients, distances


def export_file_to_config(filename):
  """
  Generates a configuration file from a CIPLEX .m source
  """
  file_base, _ = os.path.splitext(filename)
  outputname = "{}.json".format(file_base)

  with open(filename, 'r') as file:
    content = "\n".join(list(file))

    clients, distances = extract_graph(content)

    with open(outputname, 'w') as out:
      out.write(json.dumps({
        "instance_name": file_base,
        "iters": 300,
        "instance": {
          "distances": distances,
          "clients": clients,
        }
      }))


def main(args):
  if not args:
    sys.stderr.write("Error: no files provided\n")
    sys.exit(1)

  for filename in args:
    try:
      export_file_to_config(filename)
    except InfoNotFoundError as e:
      log_err("Error in file {}: {}".format(filename, e))


if __name__ == '__main__':
  main(sys.argv[1:])

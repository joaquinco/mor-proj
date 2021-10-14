import argparse
import csv
import json
import re
import os
import sys

instance_name_re = r'Instance\s+name:\s+(.*)'

# 2021-02-27 12:21:19.562601685 -03:00 | INFO | thread=1 iteration=5 best_value=591.8136197786562 construction_value=936.6249597884628 weight_config=...
execution_log_re = r'([\d-]+\s[\d:]+\.\d+\s-?[\d:]+)\s\|\s(\w+)\s\|\sthread=(\d+)\siteration=(\d+)\sbest_value=([\d\.]+)\sconstruction_value=([\d\.]+)\sweight_config=(.*)'


class LoopState:
    WaitingInstance = 'WaitingInstance'
    Instance = 'Instance'


def parse_execution_log(log_filename):
    """
    Generate dictionaries with data by parsing the log.
    """
    state = LoopState.WaitingInstance

    with open(log_filename, 'r') as logfile:
        for line in logfile:
            if state == LoopState.WaitingInstance:
                instance_name_match = re.match(instance_name_re, line)
                if instance_name_match:
                    instance_name = os.path.basename(instance_name_match[1])
                    state = LoopState.Instance
                    continue
            elif state == LoopState.Instance:
                execution_log_match = re.match(execution_log_re, line)
                if execution_log_match:
                    timestamp = execution_log_match[1]
                    _log_level = execution_log_match[2]
                    _thread = execution_log_match[3]
                    iteration = execution_log_match[4]
                    sol_value = execution_log_match[5]
                    construction_value = execution_log_match[6]
                    weight_config = execution_log_match[7]

                    yield {
                        'timestamp': timestamp,
                        'instance_name': instance_name,
                        'sol_value': sol_value,
                        'construction_value': construction_value,
                        'weight_config': weight_config,
                        'iteration': iteration,
                    }
                elif 'Writing output to' in line:
                    state = LoopState.WaitingInstance


def execution_log_to_csv(log_filename, output):
    """
    Write to output buffer a csv from by parsing a logfile
    """

    writer_created = False

    for row in parse_execution_log(log_filename):
        if not writer_created:
            csv_writer = csv.DictWriter(output, fieldnames=row.keys())
            csv_writer.writeheader()
            writer_created = True

        csv_writer.writerow(row)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('log_filename')

    args = parser.parse_args(sys.argv[1:])

    execution_log_to_csv(args.log_filename, sys.stdout)


main()

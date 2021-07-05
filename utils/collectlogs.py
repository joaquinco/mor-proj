import csv
import json
import re
import os

tunning_jobs_runconfig_dir = 'runconfig'
tunning_jobs_log_dir = 'output'
log_output = 'validator_jobs_log.csv'
runconfig_output = 'tunning_jobs_runconfigs.csv'


instance_name_re = r'Instance\s+name:\s+(.*)'

# 2021-02-27 12:21:19.562601685 -03:00 | INFO | thread=1 iteration=5 best_value=591.8136197786562 construction_value=936.6249597884628 weight_config=...
execution_log_re = r'([\d-]+\s[\d:]+\.\d+\s-?[\d:]+)\s\|\s(\w+)\s\|\sthread=(\d+)\siteration=(\d+)\sbest_value=([\d\.]+)\sconstruction_value=([\d\.]+)\sweight_config=(.*)'

class LoopState:
  WaitingInstance = 'WaitingInstance'
  Instance = 'Instance'

def add_tunning_logs(csv_writer, job_run_numbers, tunning_job_dir):
  """
  Parse tunning job execution log and add them to csv
  """
  job_name = os.path.basename(tunning_job_dir)
  match = re.match(r'validator_job_(\d+)', job_name)
  if not match:
    print(f'Ignoring unexpected tunning job dir {tunning_job_dir}')
    return

  slurm_job_id = match[1]
  job_config_id = 0
  
  job_run_numbers[job_config_id] = job_run_numbers.get(job_config_id, 0) + 1
  job_run_number = job_run_numbers[job_config_id]
  instance_name = None

  state = LoopState.WaitingInstance

  with open(f'{tunning_job_dir}/execution.log', 'r') as logfile:
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

          csv_writer.writerow({
            'slurm_job_id': slurm_job_id,
            'job_run_number': job_run_number,
            'timestamp': timestamp,
            'instance_name': instance_name,
            'sol_value': sol_value,
            'construction_value': construction_value,
            'weight_config': weight_config,
            'iteration': iteration,
          })
        elif 'Writing output to' in line:
          state = LoopState.WaitingInstance

def collect_logs():
  """
  Collect execution logs into csv
  """
  print(f'Collecting execution logs into {log_output}')

  output_logfile = open(log_output, 'w')
  fieldnames = [
    'slurm_job_id',
    'job_run_number',
    'timestamp',
    'instance_name',
    'sol_value',
    'construction_value',
    'weight_config',
    'iteration'
  ]
  writer = csv.DictWriter(output_logfile, fieldnames=fieldnames)
  writer.writeheader()
  job_run_numbers = {}

  for tunning_job_dir in sorted(os.listdir(tunning_jobs_log_dir)):
    print('.', end='', flush=True)
    add_tunning_logs(writer, job_run_numbers, os.path.join(tunning_jobs_log_dir, tunning_job_dir))

  output_logfile.close()
  print()


def collect_runconfigs():
  """
  Collect runconfigs into csv
  """
  print(f'Collecting runconfigs into {runconfig_output}')

  output_file = open(runconfig_output, 'w')
  fieldnames = [
    'job_config_id',
    'time_weight',
    'distance_weight',
    'wait_time_weight',
  ]
  writer = csv.DictWriter(output_file, fieldnames=fieldnames)
  writer.writeheader()
  rows = []

  for tunning_job_config in sorted(os.listdir(tunning_jobs_runconfig_dir)):
    job_name = os.path.basename(tunning_job_config)
    match = re.match(r'tunning_job_(\d+)', job_name)
    if not match:
      print(f'Ignoring unexpected runconfig dir {tunning_job_config}')
      continue

    job_config_id = match[1]
    with open(os.path.join(tunning_jobs_runconfig_dir, tunning_job_config), 'r') as configfile:
      config = json.loads(configfile.read())
      grasp_config = config['grasp_config']

      rows.append({
        'job_config_id': job_config_id,
        'time_weight': grasp_config['time_weight'],
        'distance_weight': grasp_config['distance_weight'],
        'wait_time_weight': grasp_config['wait_time_weight'],
      })

  rows.sort(key=lambda r: int(r['job_config_id']))
  for row in rows: writer.writerow(row)

  output_file.close()


def main():
  collect_logs()
  collect_runconfigs()

main()

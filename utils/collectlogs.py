import csv
import re
import os

tunning_jobs_log_dir = 'output'
log_output = 'tunning_jobs_log.csv'


"""
Execution log:
Columns:
- slurm_job_id
- job_run_number
- job_config_id
- timestamp
- instance_name
- sol_value
- construction_value
- iteration
"""
instance_name_re = r'Instance\s+name:\s+(.*)'

# 2021-02-27 12:21:19.562601685 -03:00 | INFO | thread=1 iteration=5 best_value=591.8136197786562 construction_value=936.6249597884628
execution_log_re = r'([\d-]+\s[\d:]+\.\d+\s-?[\d:]+)\s\|\s(\w+)\s\|\sthread=(\d+)\siteration=(\d+)\sbest_value=([\d\.]+)\sconstruction_value=([\d\.]+)'

class LoopState:
  WaitingInstance = 'WaitingInstance'
  Instance = 'Instance'

def add_tunning_logs(csv_writer, job_run_numbers, tunning_job_dir):
  """
  Parse tunning job execution log and add them to csv
  """
  job_name = os.path.basename(tunning_job_dir)
  match = re.match(r'tunning_job_(\d+)_(\d+)', job_name)
  if not match:
    raise Exception(f'Unexpected tunning job dir {tunning_job_dir}')

  job_config_id = match[1]
  slurm_job_id = match[2]
  
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

          csv_writer.writerow({
            'slurm_job_id': slurm_job_id,
            'job_run_number': job_run_number,
            'job_config_id': job_config_id,
            'timestamp': timestamp,
            'instance_name': instance_name,
            'sol_value': sol_value,
            'construction_value': construction_value,
            'iteration': iteration,
          })
        elif 'Writing output to' in line:
          state = LoopState.WaitingInstance

def main():
  output_logfile = open(log_output, 'w')
  fieldnames = [
    'slurm_job_id',
    'job_run_number',
    'job_config_id',
    'timestamp',
    'instance_name',
    'sol_value',
    'construction_value',
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

main()
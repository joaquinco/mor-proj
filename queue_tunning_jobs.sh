#!/bin/bash

logfile='tunning_scheduled_jobs.log'

touch $logfile

for job_id in $(seq 0 21); do
    for run_idx in $(seq 100); do
        execution_name=tunning_job_${job_id}_${run_idx}
        grep ${execution_name}$ $logfile > /dev/null
        if [ $? -eq 0 ]; then
            continue
        fi

        sbatch tunning_job_${job_id}.sh
        if [ $? -eq 0 ]; then
            echo "$execution_name" >> $logfile
        else
            echo "Stopped at $execution_name"
            exit 1
        fi
    done
done

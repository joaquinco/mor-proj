#!/bin/bash

export PROJECT_HOME=$HOME/projects/mor-proj/
export JOB_HOME=$HOME/jobs/mor-proj

# Set automatically by slurm
export JOB_NAME=$SLURM_JOB_NAME

# Data needed by the run.sh
export OUTPUT_PATH="$JOB_HOME/output/$JOB_NAME"
export HEURISTIC_CONFIG="$JOB_HOME/runconfig/$JOB_NAME.json"
if [ ! -n "$DATASET_PATH" ]; then
	export DATASET_PATH="$PROJECT_HOME/datasets/flexible_window_100/*.json"
fi

export LOG_FILE="$OUTPUT_PATH/execution.log"

if [ -n "$OUTPUT_PATH" ]; then
	mkdir -p "$OUTPUT_PATH"
fi

source ~/.bashrc
pyenv-init
pyenv activate mor-proj

TMP_LOG_FILE=$(mktemp -u -p /scratch/joaquin.correa/ --suffix "_$JOB_NAME-$(date --iso-8601).log")

cd $PROJECT_HOME

./bin/run.sh $DATASET_PATH > $TMP_LOG_FILE 2>&1

python utils/summarize.py $OUTPUT_PATH/*.json > $OUTPUT_PATH/summary_$JOB_HOME.csv

cp $TMP_LOG_FILE $LOG_FILE

cd -

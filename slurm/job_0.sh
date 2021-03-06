#!/bin/bash

#SBATCH --job-name=job_0
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH --mem=512
#SBATCH --time=09:00:00
#SBATCH --mail-type=ALL
#SBATCH --mail-user=joaquin.correa@fing.edu.uy

./runner.sh

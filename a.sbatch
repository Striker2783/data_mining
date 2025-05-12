#!/bin/sh

#SBATCH --job-name=test
#SBATCH --time=1:00
#SBATCH --partition=node
#SBATCH --output=/home/%u/out/%x-%u-%j.out
#SBATCH --nodes=1

srun singularity run test.sif
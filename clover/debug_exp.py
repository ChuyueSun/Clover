"""
Debug version of the experiment script focusing on a single example
"""
import os
import argparse
from sglang import OpenAI, set_default_backend

from clover import clover
from utils import (
    get_clover_anno_check_template,
    get_clover_complete_program,
    get_clover_input_sample,
)

# Parse arguments
parser = argparse.ArgumentParser()
parser.add_argument("--feedback-turn", type=int, default=3)
parser.add_argument("--num-trial", type=int, default=1)
parser.add_argument("--verbose", type=int, default=3)
parser.add_argument("--dafny-path", type=str, required=True)
parser.add_argument("--sample", type=str, default="abs")  # Default to 'abs' sample
parser.add_argument("--model", type=str, default="gpt-4o", help="OpenAI model to use")
args = parser.parse_args()

print(f"Using model: {args.model}")

# Set up the backend
set_default_backend(OpenAI(args.model))

# Set up paths for a single sample
sample_name = args.sample
dirpath = "../dataset/CloverBench"
program_path = os.path.join(dirpath, f"textbook_algo/{sample_name}/{sample_name}_strong.dfy")
doc_path = os.path.join(dirpath, f"textbook_algo/{sample_name}/{sample_name}_spec.txt")
input_sample_path = os.path.join(dirpath, f"textbook_algo_unit_tests/{sample_name}/{sample_name}_tests.dfy")
anno_template_path = os.path.join(dirpath, f"textbook_algo_anno/{sample_name}/{sample_name}_anno_check_template.dfy")

# Debug: Check if files exist
print(f"Checking if files exist:")
print(f"  Program: {os.path.exists(program_path)} ({program_path})")
print(f"  Doc: {os.path.exists(doc_path)} ({doc_path})")
print(f"  Input sample: {os.path.exists(input_sample_path)} ({input_sample_path})")
print(f"  Anno template: {os.path.exists(anno_template_path)} ({anno_template_path})")

if not all([
    os.path.exists(program_path),
    os.path.exists(doc_path), 
    os.path.exists(input_sample_path),
    os.path.exists(anno_template_path)
]):
    print("Error: Some files do not exist. Cannot proceed.")
    exit(1)

print(f"\nRunning experiment for {sample_name}")
print("="*80)

# Get the required components
program = get_clover_complete_program(program_path, doc_path)
input_sample = get_clover_input_sample(input_sample_path)
anno_check_template = get_clover_anno_check_template(anno_template_path)

# Run clover test
success, results = clover(
    program,
    input_sample,
    anno_check_template,
    dafny_path=args.dafny_path,
    feedback_turn=args.feedback_turn,
    num_trial=args.num_trial,
    verbose=args.verbose
)

print("\nResults:")
print(f"Success: {success}")
print(f"Detailed results: {results}") 
import argparse
import glob
import json
import os

from sglang import OpenAI, set_default_backend
from tqdm import tqdm

from clover import clover
from utils import (
    get_clover_anno_check_template,
    get_clover_complete_program,
    get_clover_input_sample,
    stream_jsonl,
    stream_jsonl_all
)


def collect_cloverbench_gt():
    dirpath = "../dataset/Dafny"
    search_dir = os.path.join(dirpath, "textbook_algo/*")
    sample_names = [
        name.strip("/").split("/")[-1]
        for name in glob.glob(search_dir)
        if os.path.isdir(name)
    ]

    gt_dataset = []
    for name in sample_names:
        program_path = os.path.join(dirpath, f"textbook_algo/{name}/{name}_strong.dfy")
        doc_path = os.path.join(dirpath, f"textbook_algo/{name}/{name}_spec.txt")
        input_sample_path = os.path.join(
            dirpath, f"textbook_algo_unit_tests/{name}/{name}_tests.dfy"
        )
        anno_template_path = os.path.join(
            dirpath, f"textbook_algo_anno/{name}/{name}_anno_check_template.dfy"
        )
        program = get_clover_complete_program(program_path, doc_path)
        input_sample = get_clover_input_sample(input_sample_path)
        anno_check_template = get_clover_anno_check_template(anno_template_path)

        sample = {
            "name": name,
            "program": program,
            "input_sample": input_sample,
            "anno_check_template": anno_check_template,
        }
        gt_dataset.append(sample)
    return gt_dataset

def collect_rust_humaneval_gt():
    current_path = os.getcwd()
    parent_path = os.path.dirname(current_path)
    rust_dataset_path = os.path.join(parent_path, "dataset", "rust_humaneval", "humaneval_rust_cleaned.jsonl")
    sample_jsonl = stream_jsonl_all(rust_dataset_path)
    gt_dataset = []
    for sample in sample_jsonl:
        new_sample = {
            "task_id": sample["task_id"],
            "program": sample["declaration"] + sample["canonical_solution"],
            "input_sample": sample["test"],
            "docstring": sample["docstring"],
            "signature": sample["signature"],
            "anno_check_template": None,
        }
        gt_dataset.append(new_sample)

    return gt_dataset
        
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--feedback-turn", type=int, default=3)
    parser.add_argument("--num-trial", type=int, default=3)
    parser.add_argument("--verbose", type=int, default=3)
    parser.add_argument("--dafny-path", type=str, required=False)
    parser.add_argument("--just-body", action="store_true")
    parser.add_argument("--language", type=str, default="dafny", choices=["dafny", "rust"])
    args = parser.parse_args()

    if args.language == "dafny":
        gt_dataset = collect_cloverbench_gt()
    else:
        gt_dataset = collect_rust_humaneval_gt()

    set_default_backend(OpenAI("gpt-4-1106-preview"))

    log = {"gt": {}}
    filename = f"{args.language}_exp_results_k_{args.num_trial}.log"
    checked_files = {}
    if os.path.exists(filename):
        with open(filename, "r") as f:
            log = json.load(f)
            checked_files = log["gt"].keys()
    for sample in tqdm(gt_dataset):
        if sample["task_id"] in checked_files:
            continue
        if args.verbose >= 1:
            print(
                f"================== running for {sample['task_id']} ===================="
            )
        res = clover(
            sample["program"],
            sample["input_sample"],
            sample["anno_check_template"],
            sample["signature"],
            sample["docstring"],
            sample["task_id"],
            dafny_path=args.dafny_path,
            feedback_turn=args.feedback_turn,
            num_trial=args.num_trial,
            verbose=args.verbose,
            just_body=args.just_body
        )
        log["gt"][sample["task_id"]] = res

        if args.verbose >= 1:
            print(log)
        with open(filename, "w") as f:
            json.dump(log, f, indent=4)
    accept = 0
    for sample_name, res in log["gt"].items():
        accept += int(res[0])
    print(
        f"ground truth pass rate: {accept}/{len(gt_dataset)} {accept / len(gt_dataset)}"
    )

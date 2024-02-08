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


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--feedback-turn", type=int, default=3)
    parser.add_argument("--num-trial", type=int, default=1)
    parser.add_argument("--verbose", type=int, default=3)
    args = parser.parse_args()

    gt_dataset = collect_cloverbench_gt()

    set_default_backend(OpenAI("gpt-4-1106-preview"))

    log = {"gt": {}}
    filename = f"exp_results_k_{args.num_trial}.log"
    checked_files = {}
    if os.path.exists(filename):
        with open(filename, "r") as f:
            log = json.load(f)
            checked_files = log["gt"].keys()
    for sample in tqdm(gt_dataset):
        if sample["name"] in checked_files:
            continue
        if args.verbose >= 1:
            print(
                f"================== running for {sample['name']} ===================="
            )
        res = clover(
            sample["program"],
            sample["input_sample"],
            sample["anno_check_template"],
            feedback_turn=args.feedback_turn,
            num_trial=args.num_trial,
            verbose=args.verbose,
        )
        log["gt"][sample["name"]] = res

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

import argparse
import glob
import json
import os

from sglang import OpenAI, set_default_backend
from tqdm import tqdm

from clover_mbpp50_naive import clover
from utils import (
    get_clover_anno_check_template,
    get_clover_mbpp_program,
    get_clover_input_sample,
    get_mbpp_program_signature
)


def collect_cloverbench_gt():
    dirpath = "../dataset/MBPP-DFY-50-legal/"
    # dirpath = "mbpp_log/"
    search_dir = os.path.join(dirpath, "src/*")
    doc_path = "../dataset/MBPP-DFY-50-legal/mbpp-dfy-50-examples-db.json"

    sample_names = [
        name.strip("/").split("/")[-1][:-4]
        for name in glob.glob(search_dir)
    ]
    gt_dataset = []
    no_log =[]
    for name in sample_names:
        print(name)
        program_path = os.path.join(dirpath, f"src/{name}.dfy")
        if not os.path.exists(program_path):
            no_log.append(name)
            continue
        input_sample_path = os.path.join(
            dirpath, f"test/{name}.dfy"
        )
        anno_template_path = os.path.join(
            dirpath, f"annotation_template/{name}.dfy"
        )
        task_id = name.split("_")[-1]
        program = get_clover_mbpp_program(program_path, doc_path, task_id)
        input_sample = get_clover_input_sample(input_sample_path)
        anno_check_template = get_clover_anno_check_template(anno_template_path)
        signature = get_mbpp_program_signature(doc_path, task_id)

        sample = {
            "name": name,
            "program": program,
            "input_sample": input_sample,
            "anno_check_template": anno_check_template,
            "signature": signature
        }
        gt_dataset.append(sample)
    return gt_dataset


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--feedback-turn", type=int, default=3)
    parser.add_argument("--num-trial", type=int, default=1)
    parser.add_argument("--verbose", type=int, default=1)
    parser.add_argument("--dafny-path", type=str, required=True)
    args = parser.parse_args()

    gt_dataset = collect_cloverbench_gt()

    set_default_backend(OpenAI("gpt-4-1106-preview"))

    log = {"gt": {}}
    filename = f"log_results_k_{args.num_trial}.log"
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
            sample["signature"],
            dafny_path=args.dafny_path,
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

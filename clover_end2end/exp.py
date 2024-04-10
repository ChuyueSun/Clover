import argparse
import glob
import json
import os
import tqdm
import sys
from concurrent.futures import ThreadPoolExecutor

import sglang as sgl
from sglang import OpenAI, assistant, gen, set_default_backend, system, user

sys.path.append("../clover")
import sys_prompts
from clover import clover, gen_body_from_doc
from equiv_tests import equiv_test_code
from utils import (
    compile_dafny,
    extract_body,
    extract_code_from_llm_output,
    get_clover_anno_check_template,
    get_clover_complete_program,
    get_clover_components,
    get_clover_input_sample,
    is_dafny_verified,
    no_compile_error,
    run_dafny,
    stream_print,
)
from stats import get_stats


def get_program_with_doc(dataset, dirpath, name, program_path=None):
    if dataset == "cloverbench":
        if program_path is None:
            program_path = os.path.join(dirpath, f"textbook_algo/{name}/{name}_strong.dfy")
        doc_path = os.path.join(dirpath, f"textbook_algo/{name}/{name}_spec.txt")
        return get_clover_complete_program(program_path, doc_path)
    elif dataset == "mbpp":
        data_file = os.path.join(dirpath, "mbpp-dfy-50-examples-db.json")
        with open(data_file, "r") as f:
            data = json.load(f)
        task_id = name.split("_")[-1]
        doc = ["/* " + data[task_id]["task_description"] + "*/\n"]
#         doc.append("/* " + data[task_id]["specification"]["preconditions"] + "*/\n")
#         doc.append("/* " + data[task_id]["specification"]["postconditions"] + "*/\n")

        if program_path is None:
            program_path = os.path.join(dirpath, f"src/{name}.dfy")
        with open(program_path, "r") as f:
            program = f.readlines()
        return doc + program


def collect_gt(dataset, dirpath):
    if dataset == "cloverbench":
        search_dir = os.path.join(dirpath, "textbook_algo/*")
        sample_names = [
            name.strip("/").split("/")[-1]
            for name in glob.glob(search_dir)
            if os.path.isdir(name)
        ]

        gt_dataset = []
        for name in sample_names:
            if name in ["match", "double_array_elements"]:
                continue

            input_sample_path = os.path.join(
                dirpath, f"textbook_algo_unit_tests/{name}/{name}_tests.dfy"
            )
            anno_template_path = os.path.join(
                dirpath, f"textbook_algo_anno/{name}/{name}_anno_check_template.dfy"
            )
            program = get_program_with_doc(dataset, dirpath, name)
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

    elif dataset == "mbpp":
        search_dir = os.path.join(dirpath, "src/*")
        sample_names = [
            name.strip("/").split("/")[-1][:-len(".dfy")]
            for name in glob.glob(search_dir)
        ]

        gt_dataset = []
        for name in sample_names:
            input_sample_path = os.path.join(
                dirpath, f"test/{name}.dfy"
            )
            anno_template_path = os.path.join(
                dirpath, f"annotation_template/{name}.dfy"
            )
            program = get_program_with_doc(dataset, dirpath, name)
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


@sgl.function
def gen_from_doc(s, doc, head, dafny_path, feedback_turn=3):
    # s += system(sys_prompts.SYS_DAFNY)
    s += user(sys_prompts.GEN_BODY_AND_ANNO_FROM_DOC + doc + "\n" + head)
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("artifact", max_tokens=1024))
            artifact = extract_code_from_llm_output(tmp["artifact"])
        s += assistant(artifact)
        out = compile_dafny(artifact, dafny_path)
        if no_compile_error(str(out)):
            return artifact
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."
    return artifact


def gen_gpt4_candidates_single_sample(
        sample, num_cand, exist_cands,
        correct_and_anno_sound_dirpath, correct_not_anno_sound_dirpath,
        incorrect_and_anno_sound_dirpath, incorrect_not_anno_sound_dirpath,
        dafny_path, feedback_turn=3, verbose=1
):
    name = sample["name"]
    for cand_id in range(num_cand):
        if f"{name}_{cand_id}.dfy" in exist_cands:
            print(f"======= skip {name} candidate {cand_id} ==========")
            continue
        else:
            print(f"======= processing {name} candidate {cand_id} ==========")

        # gen artifact
        doc, spec, body = get_clover_components(sample["program"])
        head = body.split("\n")[0]
        s = gen_from_doc(
            doc, head, dafny_path, feedback_turn=feedback_turn, stream=(
                verbose >= 2)
        )
        if verbose >= 2:
            stream_print(s)
        artifact = str(s.ret_value)

        # check the code correctness by unit tests
        input_sample = sample["input_sample"]
        gt = "".join(sample["program"])
        correct = equiv_test_code(artifact, gt, input_sample, dafny_path, verbose=verbose)

        # check the annotation soundness
        out, err = run_dafny(artifact, dafny_path)
        sound = is_dafny_verified(str(out))

        # classify and write to files
        if correct and sound:
            filepath = os.path.join(correct_and_anno_sound_dirpath, f"{name}_{cand_id}.dfy")
        elif correct and not sound:
            filepath = os.path.join(correct_not_anno_sound_dirpath, f"{name}_{cand_id}.dfy")
        elif not correct and sound:
            filepath = os.path.join(incorrect_and_anno_sound_dirpath, f"{name}_{cand_id}.dfy")
        elif not correct and not sound:
            filepath = os.path.join(incorrect_not_anno_sound_dirpath, f"{name}_{cand_id}.dfy")
        with open(filepath, "w") as f:
            f.write(artifact)


def gen_gpt4_candidates(dataset, dirpath, dafny_path, feedback_turn=3, verbose=1):
    correct_and_anno_sound_dirpath = os.path.join(dirpath, "gpt4_correct_and_anno_sound")
    correct_not_anno_sound_dirpath = os.path.join(dirpath, "gpt4_correct_not_anno_sound")
    incorrect_and_anno_sound_dirpath = os.path.join(dirpath, "gpt4_incorrect_and_anno_sound")
    incorrect_not_anno_sound_dirpath = os.path.join(dirpath, "gpt4_incorrect_not_anno_sound")
    os.makedirs(correct_and_anno_sound_dirpath, exist_ok=True)
    os.makedirs(correct_not_anno_sound_dirpath, exist_ok=True)
    os.makedirs(incorrect_and_anno_sound_dirpath, exist_ok=True)
    os.makedirs(incorrect_not_anno_sound_dirpath, exist_ok=True)
    num_sample = len(dataset)
    num_cand = 10
    num_threads = 8

    # extract existing candidates, and skip
    categories = ["gpt4_correct_and_anno_sound", "gpt4_correct_not_anno_sound",
                  "gpt4_incorrect_and_anno_sound", "gpt4_incorrect_not_anno_sound"]
    exist_cands = sum(get_stats(dirpath, categories), [])

    pbar = tqdm.tqdm(total=num_sample)

    with ThreadPoolExecutor(num_threads) as executor:
        futures = []
        for sample in dataset[:num_sample]:
            futures.append(
                executor.submit(
                    gen_gpt4_candidates_single_sample,
                    sample, num_cand, exist_cands,
                    correct_and_anno_sound_dirpath, correct_not_anno_sound_dirpath,
                    incorrect_and_anno_sound_dirpath, incorrect_not_anno_sound_dirpath,
                    dafny_path, feedback_turn, verbose
                )
            )
            futures[-1].add_done_callback(lambda _: pbar.update())

        rets = [f.result() for f in futures]

    pbar.close()
            

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--feedback-turn", type=int, default=3)
    parser.add_argument("--dafny-path", type=str, required=True)
    parser.add_argument("--verbose", type=int, default=1)
    args = parser.parse_args()

    set_default_backend(OpenAI("gpt-4-1106-preview"))

    for dataset in ["cloverbench", "mbpp"]:
        if dataset == "cloverbench":
            dirpath = "../dataset/CloverBench"
        elif dataset == "mbpp":
            dirpath = "../dataset/MBPP-DFY-50-legal"
        gt_dataset = collect_gt(dataset, dirpath)
        if dataset == "cloverbench":
            assert len(gt_dataset) == 60
        if dataset == "mbpp":
            assert len(gt_dataset) == 28

        gen_gpt4_candidates(gt_dataset, dirpath, args.dafny_path,
                            feedback_turn=args.feedback_turn, verbose=args.verbose)

        # collect hard examples
        path = os.path.join(dirpath, "gpt4_incorrect_and_anno_sound/*.dfy")
        files = [filepath for filepath in glob.glob(path)]
        hard_wrong_samples = []
        for file in files:
            name = "_".join(file.strip("/").split("/")[-1].split("_")[:-1])
            program = get_program_with_doc(dataset, dirpath, name, program_path=file)
            found = False
            for gt in gt_dataset:
                if gt["name"] == name:
                    hard_wrong_samples.append(
                        {
                            "name": file.strip("/").split("/")[-1][:-len(".dfy")],
                            "program": program,
                            "input_sample": gt["input_sample"],
                            "anno_check_template": gt["anno_check_template"],
                        }
                    )
                    found = True
                    break
            assert found
        print("num hard samples", len(hard_wrong_samples))
 
        # run clover on hard examples
        log = {}
        checked_samples = {}
        logfile = f"{dataset}_gpt4_wrong_samples.result"
        if os.path.exists(logfile):
            with open(logfile, "r") as f:
                log = json.load(f)
                checked_samples = log.keys()
 
        for sample in hard_wrong_samples:
            if sample["name"] in checked_samples:
                continue
            log[sample["name"]] = clover(
                sample["program"],
                sample["input_sample"],
                sample["anno_check_template"],
                args.dafny_path,
                num_trial = 5,
                verbose=args.verbose,
            )
        print(log)
        with open(logfile, "w") as f:
            json.dump(log, f, indent=4)

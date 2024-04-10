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
from clover import gen_body_from_doc
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


def collect_gt(dataset):
    if dataset == "cloverbench":
        dirpath = "../dataset/CloverBench"
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
    elif dataset == "mbpp":
        pass


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
        input_sample_path = f"../dataset/CloverBench/textbook_algo_unit_tests/{name}/{name}_tests.dfy"
        input_sample = get_clover_input_sample(input_sample_path)
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

    for dataset in ["cloverbench"]:
        gt_dataset = collect_gt(dataset)
        assert len(gt_dataset) == 60

        gen_gpt4_candidates(gt_dataset, "../dataset/CloverBench", args.dafny_path,
                            feedback_turn=args.feedback_turn, verbose=args.verbose)

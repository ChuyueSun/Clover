import argparse
import os
from typing import Dict, List

import sglang as sgl
from sglang import OpenAI, assistant, gen, set_default_backend, system, user

from equiv_tests import equiv_test_code, equiv_test_doc, equiv_test_spec
from sys_prompts import *
from utils import (
    compile_dafny,
    extract_body,
    extract_code_from_llm_output,
    extract_spec,
    get_clover_anno_check_template,
    get_clover_complete_program,
    get_clover_components,
    get_clover_input_sample,
    is_dafny_verified,
    merge_pre_and_body,
    merge_spec_and_body,
    no_compile_error,
    run_dafny,
    stream_print,
)


@sgl.function
def gen_doc_from_body(s, body):
    s += system(SYS_DAFNY)
    s += user(GEN_DOC_FROM_BODY + body)
    s += assistant(gen("new_doc", max_tokens=128))
    return s["new_doc"]


@sgl.function
def gen_doc_from_spec(s, spec):
    s += system(SYS_DAFNY)
    s += user(GEN_DOC_FROM_SPEC + spec)
    s += assistant(gen("new_doc", max_tokens=128))
    return s["new_doc"]


@sgl.function
def gen_body_from_doc(s, doc, head, input_sample, feedback_turn=3):
    s += system(SYS_DAFNY)
    s += user(GEN_BODY_FROM_DOC + doc + "\n" + head)
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("new_body", max_tokens=1024))
            body = extract_code_from_llm_output(tmp["new_body"])
        body = extract_body(body.strip().split("\n"), False)
        s += assistant(body)
        out = compile_dafny(body)
        if no_compile_error(str(out)):
            return body
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."
    return body


@sgl.function
def gen_body_from_spec(s, spec, feedback_turn=3):
    s += system(SYS_DAFNY)
    s += user(GEN_BODY_FROM_SPEC + spec)
    body = ""
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("body", max_tokens=1024))
            body = extract_code_from_llm_output(tmp["body"])
        body = extract_body(body.strip().split("\n"), False)
        s += assistant(body)
        out = compile_dafny(body)
        if no_compile_error(str(out)):
            return True, body
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."

    return False, body


@sgl.function
def gen_spec_from_doc(s, doc, head, feedback_turn=3):
    s += system(SYS_DAFNY)
    s += user(GEN_SPEC_FROM_DOC + doc + "\n" + head)
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("new_spec", max_tokens=512))
            spec = extract_code_from_llm_output(tmp["new_spec"])
        spec = extract_spec(spec.strip().split("\n"), False)
        s += assistant(spec)

        out = compile_dafny(spec)
        if no_compile_error(str(out)):
            return spec
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."

    return spec


def doc_to_body_reconstruct(
    doc: str, body: str, input_sample: str, feedback_turn=3, num_trial=1, verbose=0
):
    head = body.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_body_from_doc(
            doc, head, input_sample, feedback_turn=feedback_turn, stream=(verbose >= 2)
        )
        if verbose >= 2:
            stream_print(s)
        new_body = str(s.ret_value)
        if not equiv_test_code(body, new_body, input_sample, verbose=verbose):
            if verbose >= 2:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Doc -> body reconstruction failed.\n"
                )
        else:
            if verbose >= 1:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Doc -> body reconstruction succeeded.\n"
                )
            success = True
            break
    if not success and verbose >= 1:
        print(
            f"\n###### Clover Info::Doc -> body reconstruction failed for {num_trial} attempts.\n"
        )
    return success


def body_to_doc_reconstruct(doc: str, body: str, num_trial=1, verbose=0):
    head = body.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_doc_from_body(body, stream=(verbose >= 2))
        if verbose >= 2:
            stream_print(s)
        new_doc = str(s.ret_value)
        if not equiv_test_doc(doc, new_doc, head, verbose=verbose):
            if verbose >= 2:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Body -> doc reconstruction failed.\n"
                )
        else:
            if verbose >= 1:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Body -> doc reconstruction succeeded.\n"
                )
            success = True
            break
    if not success and verbose >= 1:
        print(
            f"\n###### Clover Info::Body -> doc reconstruction failed for {num_trial} attempts.\n"
        )
    return success


def doc_to_spec_reconstruct(
    doc: str, spec: str, anno_check_template: Dict[str, str], num_trial=1, verbose=0
):
    head = spec.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_spec_from_doc(doc, head, stream=(verbose >= 2))
        if verbose >= 2:
            stream_print(s)
        new_spec = str(s.ret_value)
        if not equiv_test_spec(spec, new_spec, anno_check_template, verbose=verbose):
            if verbose >= 2:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Doc -> spec reconstruction failed.\n"
                )

        else:
            if verbose >= 1:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Doc -> spec reconstruction succeeded.\n"
                )
            success = True
            break
    if not success and verbose >= 1:
        print(
            f"\n###### Clover Info::Doc -> spec reconstruction failed for {num_trial} attempts.\n"
        )
    return success


def spec_to_doc_reconstruct(doc: str, spec: str, num_trial=1, verbose=0):
    head = spec.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_doc_from_spec(spec, stream=(verbose >= 2))
        if verbose >= 2:
            stream_print(s)
        new_doc = str(s.ret_value)
        if not equiv_test_doc(doc, new_doc, head, verbose=verbose):
            if verbose >= 2:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Spec -> doc reconstruction failed.\n"
                )
        else:
            if verbose >= 1:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Spec -> doc reconstruction succeeded.\n"
                )
            success = True
            break
    if not success and verbose >= 1:
        print(
            f"\n###### Clover Info::Spec -> doc reconstruction failed for {num_trial} attempts.\n"
        )
    return success


def spec_soundness(spec: str, body: str, verbose=0):
    body_with_spec = merge_spec_and_body(spec, body)
    out, err = run_dafny(body_with_spec)
    if not is_dafny_verified(str(out)):
        if verbose >= 1:
            print("\n###### Clover Info::Dafny verifier failed.\n")
        return False
    if verbose >= 1:
        print("\n###### Clover Info::Dafny verifier passed.\n")
    return True


def spec_to_body_reconstruct(
    spec: str, body: str, input_sample: str, feedback_turn=3, num_trial=1, verbose=0
):
    # completeness (spec -> body)
    success = False
    for k in range(num_trial):
        s = gen_body_from_spec(spec, feedback_turn=feedback_turn, stream=(verbose >= 2))
        if verbose >= 2:
            stream_print(s)
        verified, new_body = s.ret_value
        if not verified:
            if verbose >= 2:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Failed to reconstruct a body that can be verified.\n"
                )
        elif not equiv_test_code(body, new_body, input_sample, verbose=verbose):
            if verbose >= 2:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Spec -> body reconstruction failed.\n"
                )
        else:
            if verbose >= 1:
                print(
                    f"\n###### Clover Info::Attempt ({k+1}) Spec -> body reconstruction succeeded.\n"
                )
            success = True
            break
    if not success and verbose >= 1:
        print(
            f"\n###### Clover Info::Spec -> body reconstruction failed for {num_trial} attempts.\n"
        )
    return success


def clover(
    program: List[str],
    input_sample,
    anno_check_template,
    feedback_turn=3,
    num_trial=1,
    verbose=0,
    early_quit=False,
):
    doc, spec, body = get_clover_components(program)
    ret = [None] * 6
    # # doc & body consistency
    # ret[0] = doc_to_body_reconstruct(
    #     doc,
    #     body,
    #     input_sample,
    #     feedback_turn=feedback_turn,
    #     num_trial=num_trial,
    #     verbose=verbose,
    # )
    # if early_quit and not ret[0]:
    #     return False, ret
    # body_with_pre = merge_pre_and_body(spec, body)
    # ret[1] = body_to_doc_reconstruct(
    #     doc, body_with_pre, num_trial=num_trial, verbose=verbose
    # )
    # if early_quit and not ret[1]:
    #     return False, ret

    # doc & spec consistency
    ret[2] = doc_to_spec_reconstruct(
        doc, spec, anno_check_template, num_trial=num_trial, verbose=verbose
    )
    if early_quit and not ret[2]:
        return False, ret
    ret[3] = spec_to_doc_reconstruct(doc, spec, num_trial=num_trial, verbose=verbose)
    if early_quit and not ret[3]:
        return False, ret

    # spec & body consistency
    ret[4] = spec_soundness(spec, body, verbose=verbose)
    if early_quit and not ret[4]:
        return False, ret
    ret[5] = spec_to_body_reconstruct(
        spec,
        body,
        input_sample,
        feedback_turn=feedback_turn,
        num_trial=num_trial,
        verbose=verbose,
    )
    if early_quit and not ret[5]:
        return False, ret
    if verbose >= 2:
        print("\n###### Final Clover Result: ", all(ret), ret)

    return all(ret), ret


# debug purpose
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--test-name", type=str, default="abs.dfy")
    parser.add_argument("--verbose", type=int, default=1)
    parser.add_argument("--early-quit", action="store_true")
    args = parser.parse_args()

    # backend = OpenAI("gpt-3.5-turbo")
    # backend = OpenAI("gpt-4")
    set_default_backend(OpenAI("gpt-4-1106-preview"))
    # set_default_backend(OpenAI("gpt-4"))
    program_path = (
        f"../dataset/Dafny/textbook_algo/{args.test_name}/{args.test_name}_strong.dfy"
    )
    program_path = (
        f"../dataset/Dafny/textbook_algo/{args.test_name}/{args.test_name}_strong.dfy"
    )
    program_with_pre_path = f"../dataset/Dafny/textbook_algo/{args.test_name}/{args.test_name}_code_with_pre.dfy"
    doc_path = (
        f"../dataset/Dafny/textbook_algo/{args.test_name}/{args.test_name}_spec.txt"
    )
    input_sample_path = f"../dataset/Dafny/textbook_algo_unit_tests/{args.test_name}/{args.test_name}_tests.dfy"
    anno_check_template_path = f"../dataset/Dafny/textbook_algo_anno/{args.test_name}/{args.test_name}_anno_check_template.dfy"

    program = get_clover_complete_program(program_path, doc_path)
    input_sample = get_clover_input_sample(input_sample_path)
    anno_check_template = get_clover_anno_check_template(anno_check_template_path)
    print(
        "Passed the Clover test?",
        clover(
            program,
            input_sample,
            anno_check_template,
            verbose=args.verbose,
            early_quit=args.early_quit,
        ),
    )

import sglang as sgl
from sglang import assistant, gen

from sys_prompts import DOC_EQUIV
from utils import (
    execute,
    extract_post,
    extract_pre,
    is_dafny_verified,
    run_dafny,
    stream_print,
)


def extract_exe_result(text):
    phrase = "Dafny program verifier did not attempt verification"
    index = text.find(phrase)
    if index != -1:
        start_index = index + len(phrase)
        return text[start_index:].strip()[:-1]
    else:
        return "Uncompiled"


def equiv_test_code(body, new_body, input_sample, dafny_path, verbose=0):
    body = str(body)
    new_body = str(new_body)
    output = execute(body, input_sample, dafny_path)
    new_output = execute(new_body, input_sample, dafny_path)
    if verbose >= 1:
        print("Below are outputs for checking code equivalence")
        print(extract_exe_result(output))
        print(extract_exe_result(new_output))
    return extract_exe_result(output) == extract_exe_result(new_output)


@sgl.function
def ask_llm_equiv_test_doc(s, doc, new_doc, head):
    with s.user():
        s += DOC_EQUIV
        s += "Below is the signature of the Dafny program:\n" + head + "\n"
        s += "Below is the first docstring:\n" + doc + "\n"
        s += "Below is the second docstring:\n" + new_doc + "\n"
    s += assistant(gen("doc_equiv"))
    return "YES" in s["doc_equiv"]


def equiv_test_doc(doc, new_doc, head, verbose=0):
    s = ask_llm_equiv_test_doc(doc, new_doc, head, stream=(verbose >= 2))
    if verbose >= 2:
        stream_print(s)
    elif verbose >= 1:
        print(s["doc_equiv"])
    return s.ret_value


def fill_template(head, spec):
    if spec is None:
        ret = head + "{\n}\n"
    elif spec != "":
        ret = head + "{\n" + spec + "\n" + "}\n"
    else:
        ret = head + "{\n" + "true\n" + "}\n"
    return ret


def equiv_test_spec(spec, new_spec, anno_check_template, dafny_path, verbose=0):
    check_template = ""
    check_template += fill_template(
        anno_check_template["pre_original"], extract_pre(spec)
    )
    check_template += fill_template(
        anno_check_template["pre_gen"], extract_pre(new_spec)
    )
    check_template += fill_template(anno_check_template["pre_eq"], None)
    check_template += fill_template(
        anno_check_template["post_original"], extract_post(spec)
    )
    check_template += fill_template(
        anno_check_template["post_gen"], extract_post(new_spec)
    )
    check_template += fill_template(anno_check_template["post_eq"], None)
    check_template += anno_check_template["helper_functions"]
    out, err = run_dafny(check_template, dafny_path)
    if verbose >= 1:
        print(str(out))
        print(str(err))
    return is_dafny_verified(str(out))

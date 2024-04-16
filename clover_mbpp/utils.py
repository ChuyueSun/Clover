import os
import re, json


def is_anno(line):
    if "requires" in line or "ensures" in line or "reads" in line or "modifies" in line:
        return True
    return False


def is_doc(line):
    if line.startswith("/*") or line.startswith("//") or line.startswith("/**") :
        return True
    return False


def extract_doc_code(lines):
    body = ""
    for line in lines:
        if is_anno(line) or "include" in line:
            continue
        body += line
    return body


def extract_doc_spec(lines) -> str:
    head_spec = ""
    for line in lines:
        if "include" in line:
            continue
        if line.strip() == "{":
            break
        head_spec += line
    head_spec += "{}\n"
    return head_spec


def extract_doc(lines) -> str:
    doc = ""
    for line in lines:
        if "include" in line:
            continue
        if line.startswith("method"):
            break
        doc += line
    return doc


def extract_spec(lines, oneline=True) -> str:
    spec = ""
    start = False
    for line in lines:

        if (
            line.strip().startswith("method")
            or line.strip().startswith("returns")
            or line.strip().startswith("function")
        ):
            start = True
            spec += line + "\n"
            continue
        if not start:
            continue
        if "include" in line or is_doc(line):
            continue
        if line.strip() == "{":
            break
        if not is_anno(line.strip()):
            if spec.endswith('\n'):
                spec = spec[:-1]
            spec += line + "\n"
        else:
            if oneline:
                spec += line
            else:
                spec += line + "\n"
    return spec


def extract_body(lines, oneline=True) -> str:
    body = ""
    start = False
    for line in lines:
        if (
            line.strip().startswith("method")
            or line.strip().startswith("returns")
            or line.strip().startswith("function")
        ):
            body += line + "\n"
            continue
        if "include" in line or is_doc(line):
            continue
        if line.strip() == "{":
            start = True
        if not start:
            continue
        if oneline:
            body += line
        else:
            body += line + "\n"
    return body


def extract_annotations(program):
    d = {"preconditions": [], "postconditions": []}
    req = r"requires\s+(.*?)(?=//|;|\n)"
    matches = re.finditer(req, program, re.MULTILINE)
    for matchNum, match in enumerate(matches, start=1):
        for groupNum in range(0, len(match.groups())):
            groupNum = groupNum + 1
            expression = match.group(groupNum).strip()
            if expression == "":
                continue
            d["preconditions"].append("(" + match.group(groupNum) + ")")

    ens = r"ensures\s+(.*?)(?=//|;|\n)"
    matches = re.finditer(ens, program, re.MULTILINE)
    for matchNum, match in enumerate(matches, start=1):
        for groupNum in range(0, len(match.groups())):
            groupNum = groupNum + 1
            expression = match.group(groupNum).strip()
            if expression == "":
                continue
            d["postconditions"].append("(" + match.group(groupNum) + ")")
    return d


def extract_pre(spec):
    anno = extract_annotations(spec)
    return " && ".join(anno["preconditions"])


def extract_post(spec):
    anno = extract_annotations(spec)
    return " && ".join(anno["postconditions"])


def get_caller_callee(name):
    caller_file = f"../dataset/CloverChain/one_pair/{name}/{name}_caller_gt.dfy"
    callee_file = f"../dataset/CloverChain/one_pair/{name}/{name}_callee_gt.dfy"
    with open(caller_file, "r") as f:
        caller = f.readlines()

    with open(callee_file, "r") as f:
        callee = f.readlines()

    return caller, callee


def get_clover_components(lines):
    doc = extract_doc(lines)
    spec = extract_spec(lines)
    body = extract_body(lines)
    return doc, spec, body


def get_clover_mbpp_program(program_file, doc_file, task_id):
    lines = []
    with open(doc_file, 'r', encoding='utf-8') as file:
        data = json.load(file)
        lines += ["// task_description: "+ data[task_id]["task_description"]]
        lines += ["\npreconditions: " + data[task_id]["specification"]["preconditions"]]
        lines += ["\npostconditions: " + data[task_id]["specification"]["postconditions"]]
        lines += ["\n*/\n"]
    with open(program_file, "r") as f:
        lines += f.readlines()

    return lines

def get_mbpp_program_signature(doc_file, task_id):
    sig = ""
    with open(doc_file, 'r', encoding='utf-8') as file:
        data = json.load(file)
        sig = data[task_id]["specification"]["method_signature"]

    return sig

def get_clover_input_sample(filepath):
    with open(filepath, "r") as f:
        lines = f.readlines()
    ret = ""
    method_cnt = 0
    for line in lines:
        if line.startswith("method"):
            method_cnt += 1
        if method_cnt >= 2:
            ret += line
    return ret


def extract_signature(line):
    ret = line.strip()
    if ret.endswith("{"):
        ret = ret[:-1]
    return ret + "\n"


def get_clover_anno_check_template(filepath):
    with open(filepath, "r") as f:
        lines = f.readlines()
    ret = {}
    key = None
    skip = False
    ret["helper_functions"] = ""
    helper_start = False
    for line in lines:
        if "predicate pre_original" in line:
            key = "pre_original"
        elif "predicate pre_gen" in line:
            key = "pre_gen"
        elif "lemma pre_eq" in line:
            key = "pre_eq"
        elif "predicate post_original" in line:
            key = "post_original"
        elif "predicate post_gen" in line:
            key = "post_gen"
        elif "lemma post_eq" in line:
            key = "post_eq"

        if line.strip() == "":
            continue

        if line.strip() == "{":
            skip = True

        if not skip:
            if key not in ret:
                ret[key] = extract_signature(line)
            else:
                ret[key] += extract_signature(line)


        if line.strip() == "}" and key in ["pre_original","pre_gen", "pre_eq", "post_original", "post_gen", "post_eq"]:
            skip = False
            if key == "post_eq":
                helper_start = True
        if line.strip()[-1] == "{" :
            skip = True
        if helper_start:
            ret["helper_functions"] += line
            key=""
        
    ret["helper_functions"] = ret["helper_functions"][1:-1]


    return ret


def merge_pre_and_body(spec, body):
    preconditions = extract_annotations(spec)["preconditions"]
    i = 0
    lines = body.strip().split("\n")
    for line in lines:
        if line.strip() == "{":
            break
        i += 1
    ret = (
        body.strip().split("\n")[: i + 1]
        + ["    assert " + pre + ";" for pre in preconditions]
        + body.strip().split("\n")[i + 1:]
    )
    return "\n".join(ret)


def merge_spec_and_body(spec, body):
    ret = spec.strip().split("\n")[:-1] + body.strip().split("\n")[1:]
    return "\n".join(ret)


def execute(body, input_sample, dafny_path):
    import subprocess

    program = body + input_sample

    tmp_file = dump_tmp_file(program)
    try:
        result = subprocess.run(
            f"{dafny_path} run --no-verify --unicode-char=false --allow-warnings " f"{tmp_file}",
            shell=True,
            capture_output=True,
            timeout=20,
        )
    except Exception as e:
        return str(e)
    return str(result.stdout)


def compile_dafny(body, dafny_path):
    import subprocess

    program = body 

    tmp_file = dump_tmp_file(program)
    try:
        result = subprocess.run(
            f"{dafny_path} /compile:0 /noVerify /deprecation:0  {tmp_file}",
            shell=True,
            capture_output=True,
        )

    except Exception as e:
        return str(e)
    return mask_file_names(str(result.stdout))


def extract_code_from_llm_output(reply):
    i = reply.find("<answer>")
    if i != -1:
        reply = reply[i + 8:]
        i = reply.find("</answer>")
        reply = reply[:i]
        return reply
    i = reply.find("```dafny")
    if i != -1:
        reply = reply[i + 8:]
        i = reply.find("```")
        reply = reply[:i]
        return reply
    i = reply.find("```Dafny")
    if i != -1:
        reply = reply[i + 8:]
        i = reply.find("```")
        reply = reply[:i]
        return reply
    i = reply.find("```")
    if i != -1:
        reply = reply[i + 3:]
        i = reply.find("```")
        reply = reply[:i]
        return reply
    return reply


def mask_file_names(text: str) -> str:
    # Define a pattern to match file paths
    file_path_pattern = re.compile(r"\b[\w/_/.]+\.dfy\b")
    # Replace all occurrences of file paths with 'foo.dfy'
    masked_text = file_path_pattern.sub("foo.dfy", text)
    return masked_text


def remove_duplicated_error(msg: str) -> str:
    output_list = msg.split("\n\n")
    new_output_list = []
    for text in output_list:
        if text in new_output_list:
            continue
        new_output_list.append(text)
    return "\n\n".join(new_output_list)


def dump_tmp_file(program):
    import time

    timestamp = time.time()
    tmp_dir = "tmp"
    os.makedirs(tmp_dir, exist_ok=True)
    tmp_file = f"{tmp_dir}/tmp_dafny_input_{timestamp}.dfy"
    with open(tmp_file, "w") as f:
        f.write(program)
    return tmp_file


def run_dafny(program, dafny_path):
    import subprocess

    tmp_file = dump_tmp_file(program)
    try:
        s = subprocess.run(
            f"{dafny_path} verify --verification-time-limit 20 {tmp_file}"
            " --boogie -randomSeedIterations:10",
            shell=True,
            capture_output=True,
            timeout=15,
        )
    except Exception as e:
        return "", str(e)
    out = s.stdout.decode().strip()
    err = s.stderr.decode().strip()
    out = mask_file_names(out)
    err = mask_file_names(err)
    out = remove_duplicated_error(out)
    return out, err


def concat_code(states, attr):
    ret = ""
    for state in states:
        ret += extract_code_from_llm_output(state[attr])
    return ret


def is_dafny_verified(msg: str):
    if "time out" in msg:
        return False
    if "verified, 0 errors" in msg and "File contains no code" not in msg:
        return True
    return False


def no_compile_error(msg: str):
    return "Error: " not in msg and "parse errors" not in msg


def stream_print(s):
    for out in s.text_iter():
        print(out, end="", flush=True)


if __name__ == "__main__":
    with open("test.dfy", "r") as f:
        code = f.read()
    out, err = run_dafny(code, "dafny")
    print(out)
    print(err)

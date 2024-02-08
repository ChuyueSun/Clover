import os
import re
global unit_tests, \
    test_files_llm_output_path, \
    test_files_path, max_iter_count, \
    anno_docstring_consistency, code2anno

anno2code = "/home/chuyues/llm4dafny/dataset/Dafny/wrong/C4/"
unit_tests = "/home/chuyues/llm4dafny/dataset/Dafny/textbook_algo_unit_tests/"



def extract_signiture(program):
    sig = program.partition('\n')[0]
    # print("sig: "+sig)
    return sig

def extract_method_body(program):
    idx1 = program.find('{')
    idx2 = program.rfind('}')
    
    res = "{\n"+program[idx1 + len('{') + 1: idx2]+"\n}"
    # print("body: "+res)
    return res

def run_one_folder():
    suffix = ".dfy"

    for (root, dirs, file) in os.walk(anno2code):
        # print(root, dirs, file)
        for f in file:
            if f.endswith("template.dfy") or f.endswith("strong.dfy") :
                continue
            
            if f[-len(suffix):]==suffix:
                test_name = f[:-len(suffix)]
                print(test_name)
                with open(f"{anno2code}{test_name}/{f}") as f:
                    generated_code = f.read()
                with open(f"{unit_tests}{test_name}/{test_name}_tests.dfy") as f:
                    content = f.read()
                with open(f"{unit_tests}{test_name}/{test_name}_check.py") as f:
                    pycheck = f.read()
                with open(f"{anno2code}/{test_name}/{test_name}_check.py", 'w') as f:
                    f.write(pycheck)
                i = content.find("method", 1)
                tests = content[i:]
                if not os.path.exists(test_name):
                    os.mkdir(test_name)
                with open(f"{anno2code}/{test_name}/{test_name}_tests.dfy", 'w') as f:
                    sig = extract_signiture(generated_code)
                    body = extract_method_body(generated_code)
                    new_code = sig +'\n'+body
                    f.write(new_code+'\n'+tests)

                        

    return


    
if __name__ == "__main__":
    run_one_folder()
    

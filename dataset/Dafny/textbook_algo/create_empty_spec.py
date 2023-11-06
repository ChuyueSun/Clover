import json
import random
import os
import shutil
import re

def extract_annotations(program):
    d = {"preconditions":[], "postconditions":[], "modifies/reads":[], "loopInv":[]}
    req= "requires.*"
    matches = re.finditer(req, program, re.MULTILINE)
    for matchNum, match in enumerate(matches, start=1):
        # print ("precondition {matchNum} was found at {start}-{end}: {match}".format(matchNum = matchNum, start = match.start(), end = match.end(), match = match.group()))
        d["preconditions"].append(match.group())
    modifies= "(modifies|reads).*"
    matches = re.finditer(modifies, program, re.MULTILINE)
    for matchNum, match in enumerate(matches, start=1):
        # print ("modifies/reads {matchNum} was found at {start}-{end}: {match}".format(matchNum = matchNum, start = match.start(), end = match.end(), match = match.group()))
        d["modifies/reads"].append(match.group())
    ens= "ensures.*[^;]"
    matches = re.finditer(ens, program, re.MULTILINE)
    for matchNum, match in enumerate(matches, start=1):
        # print ("postcondition {matchNum} was found at {start}-{end}: {match}".format(matchNum = matchNum, start = match.start(), end = match.end(), match = match.group()))
        d["postconditions"].append(match.group())
    # print(d)
    return d

def extract_annotations_string(program):
    d = extract_annotations(program)
    res = ""
    for l in d.values():
        res+="".join(l)+'\n'
    # print(res)
    return res

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
    suffix = "strong.dfy"
    new_suffix = "code_with_pre.dfy"
    for (root, dirs, file) in os.walk("."):
        for f in file:
            if f[-len(suffix):]==suffix:
                with open(root+'/'+f, 'r') as file:
                    # print(f)

                    filecontent = file.read()
                    # print(filecontent)
                    annodic = extract_annotations(filecontent)
                    if annodic['preconditions']!=[]:
                        print(f)
                        print(extract_annotations_string(filecontent))
                        with open(root+'/'+f[:-len(suffix)]+new_suffix,  'w') as file:
                            file.write(filecontent)     
                    else:
                        sig = extract_signiture(filecontent)
                        code = extract_method_body(filecontent)
                        with open(root+'/'+f[:-len(suffix)]+new_suffix,  'w') as file:
                            file.write(sig+'\n'+code )     
    return


    
if __name__ == "__main__":
    
    run_one_folder()

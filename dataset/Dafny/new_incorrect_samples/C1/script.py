import json
import random
import os
import shutil


def run_one_folder():
    home = "/home/chuyues/llm4dafny/dataset/Dafny/textbook_algo/"
    suffix = "strong.dfy"
    new_suffix="code_with_pre.dfy"

    spec_suffix = "spec.txt"
    # new_suffix = "C2.txt"
    for (root, dirs, file) in os.walk(home):
        for f in file:
            if f[-len(suffix):]==suffix:
                test_name = f[:-len(suffix)-1]
                spec = f[:-len(suffix)]+spec_suffix
                nocode_pre = f[:-len(suffix)]+new_suffix
                if not os.path.exists(test_name):
                    os.makedirs(test_name)
                    print(f"Directory {test_name} created.") 

                # shutil.copyfile(root+'/'+f, test_name+"/"+f)
                # shutil.copyfile(root+'/'+spec, test_name+"/"+f[:-len(suffix)]+new_suffix)
                shutil.copyfile(root+'/'+nocode_pre, test_name+"/"+f[:-len(suffix)]+new_suffix)

                        

    return



if __name__ == "__main__":
    
    with open("summary.json", "r") as file:
        data = json.load(file)
    total = 0
    for cate in data:
        print(cate + ":" + str(len(data[cate])))
        total += len(data[cate])


    print(f"Total examples: {str(total)}")
    # run_one_folder()
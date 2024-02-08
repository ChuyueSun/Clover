import json
import random
import os
import shutil


def run_one_folder():
    home = "/home/livia/llm4dafny/dataset/Dafny/textbook_algo/"
    new_suffix = "code_no_inv.dfy"
    suffix = "code_with_pre.dfy"
    for (root, dirs, file) in os.walk(home):
        for f in file:
            # print(f)
            if f[-len(suffix):]==suffix:
                test_name = f[:-len(suffix)-1]
                new_filename = f[:-len(suffix)]+new_suffix
                # if not os.path.exists(test_name):
                #     os.makedirs(test_name)
                #     print(f"Directory {test_name} created.") 
                # target_filename=f[:-len(suffix)]+new_suffix
                # print(target_filename)
                # with open(test_name+"/"+test_name+"_C1_category.txt", 'w') as file:
                #     file.write("")
                print(root+"/"+new_filename)
                shutil.copyfile(root+'/'+f, root+"/"+new_filename)
                # os.remove(test_name+"/"+f)
                # os.rmdir(test_name+"/")

                        

    return



if __name__ == "__main__":
    run_one_folder()
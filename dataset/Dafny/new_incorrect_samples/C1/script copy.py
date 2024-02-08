import json
import random
import os
import shutil


def run_one_folder():
    home = "/home/chuyues/llm4dafny/dataset/Dafny/textbook_algo_anno/"
    suffix = "anno_check_template.dfy"
    new_suffix = "C1.dfy"
    suffix="code_with_pre.dfy"
    suffix = ".dfy"
    new_suffix="code_with_pre.dfy"

    for (root, dirs, file) in os.walk("."):
        for f in file:
            # print(f)
            if f[-len(suffix):]==suffix:
                test_name = f[:-len(suffix)]
                # if not os.path.exists(test_name):
                #     os.makedirs(test_name)
                #     print(f"Directory {test_name} created.") 
                # target_filename=f[:-len(suffix)]+new_suffix
                # print(target_filename)
                # with open(test_name+"/"+test_name+"_C1_category.txt", 'w') as file:
                #     file.write("")
                # print(test_name)
                if f.endswith("template.dfy") or f.endswith("strong.dfy")\
                or f.endswith("tests.dfy") :
                    continue
                if f.endswith(new_suffix):
                    print(f)
                    print("newname: "+ root+"/"+root[2:]+"_"+new_suffix)
                    os.rename(root+'/'+f, root+"/"+root[2:]+"_"+new_suffix)
                # print(test_name)
                # print(f)
                # os.rename()
                # print(root+"/"+test_name+new_suffix)
                # shutil.copyfile(root+'/'+f, root+"/"+test_name+new_suffix)
                # os.remove(test_name+"/"+f)
                # os.rmdir(test_name+"/")

                        

    return



if __name__ == "__main__":
    
    # with open("summary.json", "r") as file:
    #     data = json.load(file)
    # total = 0
    # for cate in data:
    #     print(cate + ":" + str(len(data[cate])))
    #     total += len(data[cate])


    # print(f"Total examples: {str(total)}")
    run_one_folder()
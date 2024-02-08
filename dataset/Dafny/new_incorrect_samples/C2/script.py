import json
import random
import os
import shutil


def run_one_folder():
    home = "/home/livia/llm4dafny/dataset/Dafny/textbook_algo_anno/"
    C1 = os.getcwd()
    suffix = "anno_check_template.dfy"
    new_suffix = "C1.dfy"
    for (root, dirs, file) in os.walk(home):
        for f in file:
            if f[-len(suffix):]==suffix:
                test_name = f[:-len(suffix)-1]
                if not os.path.exists(C1+"/"+test_name):
                    os.mkdir(test_name)
                    print(f"Directory {test_name} created.") 
                    print(os.getcwd())
                    # target_filename=f[:-len(suffix)]+new_suffix
                    # print(target_filename)
                    # with open(test_name+"/"+test_name+"_C1_category.txt", 'w') as file:
                    #     file.write("")
                    # print(test_name)
                    # shutil.copyfile(root+'/'+f, test_name+"/"+f)
                # os.remove(test_name+"/"+f)
                # os.rmdir(test_name+"/")

                        

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
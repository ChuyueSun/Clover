import subprocess
import glob
import os

def exe_all_py_checks():
    check_result={}
    for filename in glob.glob("*/*check.dfy", recursive=True):
        os.remove(filename)
    #     temp = ""
    #     print(f"==========={filename}=========")
    #     with open(filename, 'r+') as f:
    #         content = f.read()
    #         i = content.find("predicate pre_gen")
    #         temp+=content[:i]
    #         content_l = content[i:].split('\n')
    #         content = content[i:]
    #         temp+=content_l[0]+"\n(#PRE) && \n...\n(#PRE)\n"
    #         i = content.find("}")
    #         content = content[i:]

    #         i = content.find("predicate post_gen")
    #         temp+=content[:i]
    #         content_l = content[i:].split('\n')
    #         temp+=content_l[0]+"\n{\n(#POST) && \n...\n(#POST)\n}\n\n"
    #         i = content.find("lemma post_eq")
    #         temp+=content[i:]
    #         new_file = filename[:-4]+"_template.dfy"
    #         os.remove(filename[:-4]+"template.dfy")
    #         with open(new_file, 'w') as newf:
    #             newf.write(temp)

            # print(temp)
            # f.write(temp)
            


    return check_result


if __name__ == "__main__":
    result = exe_all_py_checks()

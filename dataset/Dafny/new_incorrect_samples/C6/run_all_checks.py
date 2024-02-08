import subprocess
import glob
import os


def get_dfy_files():
    files = []
    for filename in glob.glob("**/*.dfy", recursive=True):
        files.append(filename)
    print("tot num files", len(files))
    return files

def exe_all_py_checks():
    check_result={}
    for filename in glob.glob("*/*_check.py", recursive=True):
        # output = subprocess.run(f"python3 {filename}", capture_output=True, text=True)
        print(filename)
        cwd = os.getcwd()+'/'+filename.split("/")[0]
        print(cwd)
        output = subprocess.run(['python3', filename.split("/")[1]], cwd=cwd, capture_output=True)
        if "PASSED" in str(output.stdout):
            check_result[filename.split("/")[0]] = True
            print("passed")
        else:
            check_result[filename.split("/")[0]] = False
            print("failed")

    print(check_result)
    return check_result
        # # print("stdout: "+str(output.stdout))
        # expected = str(output.stdout)
        # suffix = "_check.py"
        # i = expected.find("verification")
        # # print(i, expected[i+15:-11])
        # expected = expected[i+15:-11]
        # test_name = filename[:-len(suffix)]
        # print(test_name)
        # with open(f"{filename}") as f:

        #     lines = f.readlines()

        # # Check each line and replace the desired line
        # with open(f"{filename}", 'w') as file:
        #     for line in lines:
        #         if line.startswith("    expected_outputs ="):
        #             print(line)
        #             replaced = "    expected_outputs = '"+expected+"'\n"
        #             print(replaced)
        #             file.write(replaced)
        #         else:
        #             file.write(line)



if __name__ == "__main__":
    result = exe_all_py_checks()
    print("Total passed {}/{}".format(sum(result.values()), len(result)))
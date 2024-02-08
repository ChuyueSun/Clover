import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false update_map_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: update_map(...) = , map[four := 4, three := 3, one := 1, two := 2], \\n)(Test 2: update_map(...) = , map[blue := 2, yellow := 4, red := 1], \\n)(Test 3: update_map(...) = , map[apple := 3, banana := 5, cherry := 6], \\n)(Test 4: update_map(...) = , map[mouse := 11, elephant := 13, dog := 9, cat := 7], \\n)(Test 5: update_map(...) = , map[London := 17, Madrid := 19, NYC := 15], \\n)'

    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

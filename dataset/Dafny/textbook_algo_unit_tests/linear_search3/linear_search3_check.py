import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false linear_search3_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = "(Test 1: LinearSearch3(arr1, P1) = , 1, \\n)(Test 2: LinearSearch3(arr2, P2) = , 4, \\n)(Test 3: LinearSearch3(arr3, P3) = , 2, \\n)(Test 4: LinearSearch3(arr4, P4) = , 1, \\n)(Test 5: LinearSearch3(arr5, P5) = , 1, \\n)"
    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false binary_search_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: BinarySearch(new int[]{1, 2, 3, 4, 5}, 3) = , 5, \\n)(Test 2: BinarySearch(new int[]{10, 20, 30, 40, 50}, 20) = , 5, \\n)(Test 3: BinarySearch(new int[]{2, 4, 6, 8, 10}, 1) = , 5, \\n)(Test 4: BinarySearch(new int[]{5, 5, 5, 5, 5}, 5) = , 5, \\n)(Test 5: BinarySearch(new int[]{0}, 0) = , 1, \\n)'
    
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

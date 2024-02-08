import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false set_to_seq_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: SetToSeq({1, 2, 3, 4, 5}) = , [1, 2, 3, 4, 5], \\n)(Test 2: SetToSeq({10, 20, 30, 40}) = , [10, 20, 30, 40], \\n)(Test 3: SetToSeq({100, 200, 300}) = , [100, 200, 300], \\n)(Test 4: SetToSeq({1000, 2000, 3000, 4000}) = , [1000, 2000, 3000, 4000], \\n)(Test 5: SetToSeq({10000, 20000, 30000, 40000, 50000}) = , [10000, 20000, 30000, 40000, 50000], \\n)'

    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

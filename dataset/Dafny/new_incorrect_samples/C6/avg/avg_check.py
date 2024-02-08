import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false avg_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: ComputeAvg(2, 2) = , 4, \\n)(Test 2: ComputeAvg(7, 3) = , 10, \\n)(Test 3: ComputeAvg(100, 200) = , 300, \\n)(Test 4: ComputeAvg(0, 0) = , 0, \\n)(Test 5: ComputeAvg(-7, -2) = , -9, \\n)'
    
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

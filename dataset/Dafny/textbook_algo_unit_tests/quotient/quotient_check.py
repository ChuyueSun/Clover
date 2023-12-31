import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false quotient_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: Quotient(10, 2) = , 0, 5, \\n)(Test 2: Quotient(15, 5) = , 0, 3, \\n)(Test 3: Quotient(100, 10) = , 0, 10, \\n)(Test 4: Quotient(50, 3) = , 2, 16, \\n)(Test 5: Quotient(30, 4) = , 2, 7, \\n)'

    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false count_lessthan_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: CountLessThan(set{1, 2, 3, 4, 5}, 3) = , 2, \\n)(Test 2: CountLessThan(set{10, 20, 30, 40, 50}, 25) = , 3, \\n)(Test 3: CountLessThan(set{5, 15, 25, 35, 45}, 30) = , 2, \\n)(Test 4: CountLessThan(set{}, 5) = , 0, \\n)(Test 5: CountLessThan(set{100, 200, 300, 400, 500}, 600) = , 0, \\n)'
    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

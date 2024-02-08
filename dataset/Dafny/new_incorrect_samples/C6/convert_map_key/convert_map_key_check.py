import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false convert_map_key_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: convert_map_key(inputs1, func1) = , map[3 := true, 4 := false, 5 := true], \\n)(Test 2: convert_map_key(inputs2, func2) = , map[16 := false, 20 := false], \\n)(Test 3: convert_map_key(inputs3, func3) = , map[26 := true, 27 := false, 28 := true], \\n)(Test 4: convert_map_key(inputs4, func4) = , map[89 := true, 98 := false], \\n)(Test 5: convert_map_key(inputs5, func5) = , map[71 := true, 75 := false, 79 := true], \\n)'
    
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false swap_sim_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(Test 1: SwapSimultaneous(3,5) = , 8, 2, \\n)(Test 2: SwapSimultaneous(-1,0) = , -1, 1, \\n)(Test 3: SwapSimultaneous(0,0) = , 0, 0, \\n)(Test 4: SwapSimultaneous(100,-100) = , 0, -200, \\n)(Test 5: SwapSimultaneous(123456789,987654321) = , 1111111110, 864197532, \\n)'
    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

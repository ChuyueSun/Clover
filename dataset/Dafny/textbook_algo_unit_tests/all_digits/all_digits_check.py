import subprocess
import ast 

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false  --no-verify all_digits_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '(true, \\n)(false, \\n)(true, \\n)(false, \\n)(false, \\n)'

    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")


if __name__ == '__main__':
    run_and_check_output()

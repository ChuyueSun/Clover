import subprocess
import ast 
# Flatten function
def flatten_tuple(tup):
    # Convert characters to string and append the number
    return ''.join(tup[0]) + str(tup[1]) + ''.join(tup[2])

def run_and_check_output():
    # Run the Dafny program and capture its output
    result=subprocess.run(f"~/llm4dafny/exp/dafny/Binaries/Dafny run --no-verify --unicode-char=false array_concat_tests.dfy", shell=True, capture_output=True)

    output = str(result.stdout)[2:-1]

    result = str(output)

    print("dafny outputs: \n"+str(result))

    # Define the expected outputs
    expected_outputs = '579;-5-7-9;;Error: Execution resulted in exception: Exception has been thrown by the target of an invocation.\\nSystem.IndexOutOfRangeException: Index was outside the bounds of the array.\\n   at _module.__default.concat(BigInteger[] a, BigInteger[] b)\\n   at _module.__default.TestMethod()\\n   at _module.__default._Main(ISequence`1 __noArgsParameter)\\n   at __CallToMain.<>c__DisplayClass0_0.<Main>b__0()\\n   at Dafny.Helpers.WithHaltHandling(Action action)\\n   at __CallToMain.Main(String[] args)\\n'
    # Check and print results
    if expected_outputs in str(result):
        print("PASSED")
    else: 
        print("FAILED")

if __name__ == '__main__':
    run_and_check_output()

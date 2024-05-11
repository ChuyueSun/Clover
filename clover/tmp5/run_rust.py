import subprocess
import os
import json

def compile_and_test_rust_file(rust_file):
    """Compiles and tests a Rust file."""
    results = {'compiled': False, 'passed': False}
    # Compile the Rust file with --test to include test functions
    compile_result = subprocess.run(['rustc', '--test', rust_file, '-o', 'test_executable'], capture_output=True, text=True)
    
    if compile_result.returncode == 0:
        results['compiled'] = True
        print(f"Compilation successful for {rust_file}.")
        # Run the compiled test executable
        test_result = subprocess.run(['./test_executable'], capture_output=True, text=True)
        os.remove('test_executable')  # Clean up executable after testing
        if test_result.returncode == 0:
            results['passed'] = True
            print(f"All tests passed for {rust_file}.")
        else:
            print(f"Some tests failed for {rust_file}.")
            print(test_result.stderr)
    else:
        print(f"Compilation failed for {rust_file}.")
        print(compile_result.stderr)
    
    return results

def test_all_rust_files(directory):
    """Tests all Rust files in a specified directory and writes results to a JSON file."""
    results = {}
    for filename in os.listdir(directory):
        if filename.endswith(".rs"):
            file_path = os.path.join(directory, filename)
            results[filename] = compile_and_test_rust_file(file_path)
    
    with open('test_results.json', 'w') as f:
        json.dump(results, f, indent=4)

# Example usage:
if __name__ == "__main__":
    directory = cwd = os.getcwd()   # Path to the directory containing Rust files
    test_all_rust_files(directory)

import os
import re
import json
from pathlib import Path

def extract_first_docstring(source_code):
    """
    Extracts the first docstring from a given Rust source code.
    
    Args:
    source_code (str): The Rust source code as a string.
    
    Returns:
    str: The first docstring found within the source code, or None if no docstring is found.
    """
    # Regular expression pattern to find the first docstring
    pattern = r'/\*(.*?)\*/'
    # Using re.DOTALL to ensure that '.' matches newlines
    match = re.search(pattern, source_code, re.DOTALL)
    if match:
        return match.group(1).strip()
    else:
        return None



def extract_function_signature(source_code):
    """
    Extracts the first function signature from a Rust source code that is not named 'main'.
    
    Args:
    source_code (str): The Rust source code as a string.
    
    Returns:
    str: The first function signature found that is not 'main', or None if no such function is found.
    """
    # Regular expression pattern to find function signatures excluding 'main'
    pattern = r'\bfn\s+(?!main\b)(\w+)\s*\(([^{]*)\)'
    # Searching for the pattern in the source code
    match = re.search(pattern, source_code)
    if match:
        return match.group(0)
    else:
        return None
def rewrite_string(s):
    """
    Rewrites the given string from the format 'rust_12' to 'Rust/12'.

    Args:
    s (str): The string to rewrite.

    Returns:
    str: The rewritten string.
    """
    parts = s.split('_')  # Split the string at the underscore
    parts[0] = parts[0].capitalize()  # Capitalize the first part 'rust' -> 'Rust'
    return '/'.join(parts)  # Join the parts with a '/' instead of '_'


def update_jsonl_files(directory, jsonl_path):
    # Dictionary to store the data from jsonl file with task_id as key
    tasks = {}

    # Load current JSONL data into memory
    with open(jsonl_path, 'r') as file:
        for line in file:
            data = json.loads(line.strip())
            tasks[data['task_id']] = data

    # Traverse directory for Rust files
    for filename in os.listdir(directory):
        if filename.endswith('.rs'):
            task_id = filename[:-3]  # Assuming task_id is filename without '.rs'
            task_id = rewrite_string(task_id)
            if task_id in tasks:
                filepath = os.path.join(directory, filename)
                with open(filepath, 'r') as file:
                    source_code = file.read()

                # Extract docstring and function signature
                docstring = extract_first_docstring(source_code)
                signature = extract_function_signature(source_code)

                # Update the task dictionary
                if docstring:
                    tasks[task_id]['docstring'] = docstring
                else:
                    assert False
                if signature:
                    tasks[task_id]['signature'] = signature
                else:
                    assert False
    print(tasks["Rust/161"])

    # Write updated data back to the JSONL file
    with open(jsonl_path, 'w') as file:
        for task in tasks.values():
            file.write(json.dumps(task) + '\n')

# Example usage
directory_path = 'src/bin/'
jsonl_filename = 'humaneval_rust_cleaned.jsonl'
update_jsonl_files(directory_path, jsonl_filename)
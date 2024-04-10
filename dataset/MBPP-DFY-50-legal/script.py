import os
GT_WRONG_IDS = ["472", "567", "576", "632", "644", "803"]
CLOVER_OUT_OF_SCOPE_IDS = ["477", "798", "764", "573", "775", "566", "588", "629", "579", "784", "624", "807", "594", "755", "732"]
DOCSTRING_NOT_PRECISE_IDS = [ "602", "793"]

def delete_files_based_on_substring(root_dir, substrings_to_match):
    """
    Delete files in all subdirectories under root_dir if file name after
    splitting by "_" contains any of the specified substrings.

    Args:
        root_dir (str): The root directory to start searching from.
        substrings_to_match (list of str): Substrings to match for file deletion.
    """
    for subdir, dirs, files in os.walk(root_dir):
        for file in files:
            # Split file name by "_"
            parts = file.split("_")
            if len(parts)!=3:
                continue
            
            number = parts[2][:-4]

            
            # Check if any part matches any of the specified substrings
            if any(number==substring for substring in substrings_to_match):
                file_path = os.path.join(subdir, file)
                os.remove(file_path)  # Delete the file
                print(f"Deleted {file_path}")

# Example usage
current_directory = os.getcwd()  # Get the current working directory
substrings_to_match = GT_WRONG_IDS + CLOVER_OUT_OF_SCOPE_IDS + DOCSTRING_NOT_PRECISE_IDS # Specify substrings to match
delete_files_based_on_substring(current_directory, substrings_to_match)

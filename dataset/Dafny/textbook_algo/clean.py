import os
import glob

def delete_files_with_extension(extension, directory='.'):
    """
    Delete files with a specific extension in the specified directory and its subdirectories.

    :param extension: The extension of the files to be deleted.
    :param directory: The directory to search in. Defaults to the current directory.
    """
    # Create a pattern to match files
    pattern = os.path.join(directory, '**', f'*{extension}')
    
    # Use glob to find files with the specified extension
    files_to_delete = glob.glob(pattern, recursive=True)
    
    # Delete the files found
    for file in files_to_delete:
        print(file)
        try:
            os.remove(file)
            print(f"Deleted: {file}")
        except OSError as e:
            print(f"Error: {e.strerror}, while trying to delete {file}")

# Usage
delete_files_with_extension('foo.dfy')

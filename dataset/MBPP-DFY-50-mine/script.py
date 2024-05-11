import os


def extract_input_output(header):
    print("???", header)
    # Extracting the part between 'method function_name(' and optionally ') returns'
    input_output_parts = header.split('method ')[1].split('(')[1].split(')')[0]
    
    # Checking if there's a 'returns' part and appending it to the input part
    if 'returns' in header:
        output_part = header.split('returns (')[1].rstrip(')')
        input_output_parts += ', ' + output_part
    
    return input_output_parts


def extract_pure_input_output(header):
    if "returns" in header:
    # Splitting the header to identify method inputs and outputs
      method_definition, return_definition = header.split(" returns ")
    else:
      method_definition, return_definition = header, ""

    
    # Extracting input variable name(s)
    input_part = method_definition.split("(")[-1]  # Gets the part inside the parentheses
    input_names = [part.split(":")[0].strip() for part in input_part.split(",")]
    
    # Extracting output variable name(s)
    output_part = return_definition.strip("()")
    output_names = [part.split(":")[0].strip() for part in output_part.split(",")]
    
    # Joining input and output names
    all_names = ", ".join(input_names + output_names)
    
    return all_names

def transform_dafny_to_predicates(dafny_code):
    # Splitting the input Dafny code into lines
    lines = dafny_code.split('\n')
    lines = [line for line in lines if line!=""]
    # Extracting the method name, inputs, and outputs from the first line
    header_parts = lines[0].split()
    input_vars = extract_input_output(lines[0])
    pure_input_vars = extract_pure_input_output(lines[0])
    print(input_vars)
    method_name = header_parts[1]
    
    # Generating the predicate and lemma code
    predicates = f"""predicate pre_original({input_vars})
{{
true
}}

predicate pre_gen({input_vars})
{{
  true // (#PRE) && ... (#PRE)
}}

lemma pre_eq({input_vars})
  ensures pre_original({pure_input_vars}) <==> pre_gen({pure_input_vars})
{{
}}

predicate post_original({input_vars})
  requires pre_original({pure_input_vars})
{{
true
}}

predicate post_gen({input_vars})
  requires pre_original({pure_input_vars})
{{
  true // (#POST) && ... (#POST)
}}

lemma post_eq({input_vars})
  requires pre_original({pure_input_vars})
  requires pre_gen({pure_input_vars})
  ensures post_original({pure_input_vars}) <==> post_gen({pure_input_vars})
{{
}}"""

    return predicates



def process_files(src_dir, dest_dir):
    """
    Process files in the src_dir, applying a transformation, and writing the output to dest_dir.

    Args:
        src_dir (str): Source directory containing the files to process.
        dest_dir (str): Destination directory for the transformed files.
    """
    # Ensure destination directory exists
    os.makedirs(dest_dir, exist_ok=True)

    # Iterate over all files in the source directory
    for filename in os.listdir(src_dir):
        src_file_path = os.path.join(src_dir, filename)
        print("+++++++++", src_file_path)

        dest_file_path = os.path.join(dest_dir, filename)
        
        # Check if it's a file and not a directory
        if os.path.isfile(src_file_path):
            # Read the content of the source file
            with open(src_file_path, 'r', encoding='utf-8') as file:
                content = file.read()
            
            # Apply the transformation function to the content
            transformed_content = transform_dafny_to_predicates(content)
            
            # Write the transformed content to the destination file
            with open(dest_file_path, 'w', encoding='utf-8') as file:
                file.write(transformed_content)
            
            print(f"Processed and wrote transformed content to {dest_file_path}")




src_directory = "src/"
dest_directory = "anno_template/"
process_files(src_directory, dest_directory)
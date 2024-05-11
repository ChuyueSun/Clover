import json
import os

# Function to read JSON data and write the Dafny code to a file
def write_dafny_code_from_json(json_file_path):
    # Ensure the /dfy_files directory exists
    output_directory = "src/"
    GT = 0
    GT_task_ids = []
    total = 0
    total_ids = []

    # Open and read the JSON file
    with open(json_file_path, 'r') as file:
        data = json.load(file)
    for item in data:
    # Extract dafny_code and id
        if item["isVerified"]==False:
            continue
        task_id = item.get("id", "")
        total+=1
        total_ids.append(task_id)
        if "[STRONG-POST]" in item["note"]:
            GT+=1
            GT_task_ids.append(task_id)
        dafny_code = item.get("dafny_code", "")
        
        if "predicate" in dafny_code:
            continue
        if "function" in dafny_code:
            continue

        # # Create the output file path
        # output_file_path = os.path.join(output_directory, f"task_id_{task_id}.dfy")

        # # Write the dafny_code to the output file
        # with open(output_file_path, 'w') as output_file:
        #     output_file.write(dafny_code)

        # print(f"Dafny code written to {output_file_path}")
    print("total verified: ", str(total))
    print("total GT: ", str(GT), str(GT/total))
    print("total id list: ", total_ids)
    print("GT id list: ", GT_task_ids)

def extract_typed_var_names(signature):
    """
    Extract typed variable names from a method signature in Dafny.

    Args:
        signature (str): The method signature from which to extract variable names and types.

    Returns:
        str: A string containing the typed variable names, separated by commas.
    """
    try:
        parts = signature.split("returns")
        input_part = parts[0].split("(")[1].rstrip(") ")
        if len(parts)==2:
            output_part = parts[1].strip(" ()")
        # input_part = signature.split("(")[1].split(")")[0]
            # output_part = signature.split("returns (")[1].rstrip(")")
            combined_parts = f"{input_part}, {output_part}"
        else:
            combined_parts = f"{input_part}"

        
    except:
        combined_parts = ""

    return combined_parts
def transform_dafny_to_predicates(dafny_code):
    # Splitting the input Dafny code into lines
    lines = dafny_code.split('\n')
    lines = [line for line in lines if (line!="")]
    lines = [line for line in lines if not line.startswith("/*")]

    # Extracting the method name, inputs, and outputs from the first line
    # header_parts = lines[0].split()
    print(lines[0])
    typed_input_vars =extract_typed_var_names(lines[0])
    array_names = extract_array_var_names(typed_input_vars)
    input_vars = extract_input_output_names(lines[0])

    if array_names:
        print("????? ", array_names)
           
        
        # Generating the predicate and lemma code
        predicates = f"""twostate predicate pre_original({typed_input_vars})
    reads {array_names}
    {{
    true
    }}

    twostate predicate pre_gen({typed_input_vars})
    reads {array_names}

    {{
    true // (#PRE) && ... (#PRE)
    }}

    twostate lemma pre_eq({typed_input_vars})
    ensures pre_original({input_vars}) <==> pre_gen({input_vars})
    {{
    }}

    twostate predicate post_original({typed_input_vars})
    reads {array_names}
    requires pre_original({input_vars})
    {{
    true
    }}

    twostate predicate post_gen({typed_input_vars})
    reads {array_names}
    requires pre_original({input_vars})
    {{
    true // (#POST) && ... (#POST)
    }}

    twostate lemma post_eq({typed_input_vars})
    requires pre_original({input_vars})
    requires pre_gen({input_vars.split(':')[0]})
    ensures post_original({input_vars.split(':')[0]}) <==> post_gen({input_vars.split(':')[0]})
    {{
    }}"""

        return predicates

    else:
                    
        # Generating the predicate and lemma code
        predicates = f"""twostate predicate pre_original({typed_input_vars})
    {{
    true
    }}

    twostate predicate pre_gen({typed_input_vars})
    {{
    true // (#PRE) && ... (#PRE)
    }}

    twostate lemma pre_eq({typed_input_vars})
    ensures pre_original({input_vars}) <==> pre_gen({input_vars})
    {{
    }}

    twostate predicate post_original({typed_input_vars})
    requires pre_original({input_vars})
    {{
    true
    }}

    twostate predicate post_gen({typed_input_vars})
    requires pre_original({input_vars})
    {{
    true // (#POST) && ... (#POST)
    }}

    twostate lemma post_eq({typed_input_vars})
    requires pre_original({input_vars})
    requires pre_gen({input_vars.split(':')[0]})
    ensures post_original({input_vars.split(':')[0]}) <==> post_gen({input_vars.split(':')[0]})
    {{
    }}"""

        return predicates



def extract_input_output_names(header):
    # Splitting the header to identify method inputs and outputs
    try: 
        method_definition, return_definition = header.split(" returns ")
    except:
        return ""
    
    # Extracting input variable name(s)
    input_part = method_definition.split("(")[-1]  # Gets the part inside the parentheses
    input_names = [part.split(":")[0].strip() for part in input_part.split(",")]
    
    # Extracting output variable name(s)
    output_part = return_definition.strip("()")
    output_names = [part.split(":")[0].strip() for part in output_part.split(",")]
    
    # Joining input and output names
    all_names = ", ".join(input_names + output_names)
    
    return all_names

def extract_array_var_names(typed_vars):
    """
    Extract variable names of type array from a string containing typed variable declarations.

    Args:
        typed_vars (str): A string containing typed variable declarations, separated by commas.

    Returns:
        str: A comma-separated string containing only the names of variables that are of array type.
    """
    # Splitting the string into individual variable declarations
    print(typed_vars)
    vars_list = typed_vars.split(", ")
    print(vars_list)
    try:
    # Filtering for variables that are of array type
        array_vars = [var.split(":")[0] for var in vars_list if "array" in var.split(":")[1]]
    except:
        array_vars = []
    # Joining the names of array variables with commas
    array_var_names = ", ".join(array_vars)
    
    return array_var_names


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
        print("======="+filename)
        src_file_path = os.path.join(src_dir, filename)
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

json_path = "rq3-dynamic-few-shot-prompting-GPT-4-temp_0.5-verified-unverified-tagged.json"
# Example usage
src_directory = "src/"
dest_directory = "annotation_template/"
# Transform the Dafny code
# write_dafny_code_from_json(json_path)
# process_files(src_directory, dest_directory)

write_dafny_code_from_json(json_path)
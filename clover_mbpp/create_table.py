import json

# Function to process the JSON data
def process_json(file_path):
    table  = ""
    # Open and read the JSON data from a file
    with open(file_path, 'r') as file:
        parsed_data = json.load(file)
    
    # Access the 'gt' dictionary
    gt_data = parsed_data['gt']
    num_tasks = 0
    for task_id, values in gt_data.items():
        num_tasks +=1
        overall_status = values[0]
        individual_statuses = values[1]
        
        print(f"Task ID: {task_id}")
        print(f"Overall Status: {overall_status}")
        print("Individual Statuses:")
        task_id = task_id.replace("_", "\\_")

        table+="\stepcounter{rowcount}\\arabic{rowcount} &"+task_id
        for t in individual_statuses:
            
            if t:
                table+=" & A "
            else:
                table+=" & R "
        table+=" \\\ \hline\n"        
        for index, status in enumerate(individual_statuses, start=1):
            print(f"  Step {index}: {'Completed' if status else 'Not Completed'}")
        
    print(table)
    print("total number of tasks: ", num_tasks)

# Path to the JSON file
file_path = 'log_results_k_10.log'

# Execute the function with the path to the JSON file
process_json(file_path)


def create_table(result_table):
    table = ""
    keys = sorted(result_table[0].keys())
    for k in sorted(keys):
        name = k.replace("_", "\\_")
        table+="\stepcounter{rowcount}\\arabic{rowcount} &"+name
        for t in result_table:
            
            if t[k]:
                table+=" & A "
            else:
                table+=" & R "
        table+=" \\\ \hline\n"
    print(table)
    return table
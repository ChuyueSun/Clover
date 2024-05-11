import os
import json

def summarize_results(directory):

    summary={}

    for root, dirs, files in os.walk(directory):
        for file in files:
            if file == "test_results.json":
                filepath = os.path.join(root, file)
                with open(filepath, 'r') as f:
                    data = json.load(f)
                    for field_name, values in data.items():
                        if field_name not in summary:
                            summary[field_name] = values
                            continue

                        summary[field_name]['compiled']=summary[field_name]['compiled']|values["compiled"]
                        summary[field_name]['passed']=summary[field_name]['passed']|values["passed"]
    return summary

# Specify the root directory to start searching
root_directory = os.getcwd()

summary = summarize_results(root_directory)

with open('test_results_summary.json', 'w') as f:
    json.dump(summary, f, indent=4)

with open("tmp1/test_results.json", 'r') as f:
    summary = json.load(f)

passed = 0
compiled = 0
for field_name, values in summary.items():
    if values["compiled"]:
        compiled+=1
    if values["compiled"] and values["passed"]:
        passed+=1
print("Total compiled: ", compiled)
print("Total passed: ", passed)
print("total: ", len(summary))

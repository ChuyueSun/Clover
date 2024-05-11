import json

# Assuming the JSON data is stored in a file named 'data.json'.
with open('rq3-dynamic-few-shot-prompting-GPT-4-temp_0.5-verified-unverified-tagged.json', 'r') as file:
    data = json.load(file)

# The rest of the code remains the same.
for item in data:
    # Check if 'isVerified' is True and '[CONFIRMED-WITH-TESTS]' is not in 'note'.
    if item["isVerified"] and "[CONFIRMED-WITH-TESTS]" not in item["note"]:
        print(item["id"])  # or process the item as needed.
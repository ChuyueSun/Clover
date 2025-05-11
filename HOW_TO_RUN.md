# How to Run Clover Experiments

## 1. Activate the Environment
```bash
source clover_env/bin/activate
```

## 2. Set Your OpenAI API Key
```bash
export OPENAI_API_KEY=your_api_key_here
```
Replace `your_api_key_here` with your actual OpenAI API key.

### Alternative using Private Key File (recommended)
```bash
# Create the private key file 
echo '#!/bin/bash
export OPENAI_API_KEY="your-actual-api-key-here"' > private_key.sh

# Make it executable and source it
chmod +x private_key.sh
source private_key.sh
```

## 3. Run the Experiments

### Basic Run
```bash
cd clover
python exps.py --num-trial 1 --dafny-path $(which dafny)
```

### For Linux Users
If you encounter issues with shell syntax on Linux:
```bash
# Explicitly provide the dafny path if $(which dafny) doesn't work
cd clover
python exps.py --num-trial 1 --dafny-path /usr/local/dafny/dafny  # Adjust path as needed
```

### Advanced Options
```bash
# Debug mode with more verbose output
python debug_exp.py --num-trial 1 --verbose 2 --model gpt-4o --dafny-path $(which dafny)
```

You can adjust the `--num-trial` parameter based on your needs. Start with a small number (like 1) for testing purposes.

## 4. Troubleshooting

### OpenAI API Issues
If you encounter API errors like `invalid_parameter_combination`:
1. Check your API key format and make sure it doesn't contain unexpected characters
2. Try using a private key file as shown above

### Dafny Path Issues
If you get an error about Dafny not being found:
1. Verify Dafny is installed: `which dafny`
2. Make sure it's in your PATH
3. Provide the full path to the Dafny executable with the `--dafny-path` parameter

## 5. Full Example
```bash
# Activate environment
source clover_env/bin/activate

# Set API key (replace with your actual key)
export OPENAI_API_KEY=sk-your-actual-key-here

# Run with 1 trial (for testing)
cd clover
python exps.py --num-trial 1 --dafny-path $(which dafny)

# Or run with more trials once you confirm it's working
# python exps.py --num-trial 50 --dafny-path $(which dafny)
``` 
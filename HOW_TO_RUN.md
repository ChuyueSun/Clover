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

## 3. Run the Experiments
```bash
cd clover
python exps.py --num-trial 1 --dafny-path $(which dafny)
```

You can adjust the `--num-trial` parameter based on your needs. Start with a small number (like 1) for testing purposes.

## 4. Full Example
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
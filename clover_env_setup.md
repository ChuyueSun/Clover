# Clover Environment Setup Guide

## 1. Virtual Environment Setup

A Python virtual environment has been created in the `clover_env` directory with all the required Python dependencies.

To activate the environment:
```bash
source clover_env/bin/activate
```

## 2. Installing Dafny

Dafny is required to run the experiments. You need to install it before running the code:

### macOS (using Homebrew)
```bash
brew install dafny
```

### Manual Installation
If Homebrew method doesn't work:
1. Download Dafny from the official GitHub repository: https://github.com/dafny-lang/dafny/releases
2. Extract the archive to a location on your machine
3. Add the Dafny binary directory to your PATH

## 3. Running the Experiments

After installing Dafny, you can run the experiments with:

```bash
# Activate the environment first
source clover_env/bin/activate

# Set your OpenAI API Key
export OPENAI_API_KEY=your_api_key_here

# Run the experiments
cd clover
python exps.py --num-trial 50 --dafny-path $(which dafny)
```

## 4. Notes

- The `--dafny-path` argument must point to your Dafny executable.
- You may adjust the `--num-trial` parameter as needed.
- Your OpenAI API key must be set in the environment variable for the experiments to work. 
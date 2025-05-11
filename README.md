# Clover: Closed-Loop Verifiable Code Generation [[paper](https://arxiv.org/abs/2310.17807)]

<p align="center">
<img src="figures/clover_overview.jpg" alt="clover_overview" width="500"/>
</p>

## Abstract
The use of large language models for code generation is a rapidly growing trend in software development. 
However, without effective methods for ensuring the correctness of generated code, this trend could lead to any number of undesirable outcomes. 
In this paper, we lay out a vision for addressing this challenge: the Clover paradigm, short for Closed-Loop Verifiable Code Generation, 
which reduces correctness checking to the more accessible problem of consistency checking. At the core of Clover lies a checker 
that performs consistency checks among code, docstrings, and formal annotations. The checker is implemented using a novel integration of formal verification tools and large language models. We provide a theoretical analysis to support our thesis that Clover should be effective at consistency checking. 
We also empirically investigate its feasibility on a hand-designed dataset (CloverBench) featuring annotated Dafny programs
 at a textbook level of difficulty. Experimental results show that for this dataset, (i) LLMs are reasonably successful at 
 automatically generating formal specifications; and (ii) our consistency checker achieves a promising acceptance rate 
 (up to 87%) for correct instances while maintaining zero tolerance for incorrect ones (no false positives).

## Setup Guide

### 1. Environment Setup

#### Option 1: Using Python venv
```bash
# Create and activate a virtual environment
python3 -m venv clover_env
source clover_env/bin/activate

# Install requirements
pip install "sglang[openai]" tqdm
```

#### Option 2: Using the provided configuration
We've created a ZSH configuration file to make working with Clover easier:
```bash
source .clover_zshrc
```

### 2. Install Dafny
Dafny is required to run the Clover experiments. You can install it with:

#### macOS
```bash
brew install dafny
```

#### Other platforms
Download from the [Dafny GitHub releases](https://github.com/dafny-lang/dafny/releases) and add it to your PATH.

### 3. Set OpenAI API Key
Set your OpenAI API key as an environment variable:
```bash
export OPENAI_API_KEY=your_api_key_here
```

#### Recommended: Use a private key file (not committed to git)
For security, create a private key file that won't be committed to git:
```bash
# Create the private key file
cat > private_key.sh << 'EOF'
#!/bin/bash
export OPENAI_API_KEY="your-actual-api-key-here"
EOF

# Make it executable
chmod +x private_key.sh

# Use it
source private_key.sh
```

This approach is automatically supported by the included `.clover_zshrc` and `set_openai_key.sh` scripts.

### 4. Run Experiments

#### Quick Testing
```bash
cd clover
python exps.py --num-trial 1 --dafny-path $(which dafny) --model gpt-4o
```

#### Full Experiment
```bash
cd clover
python exps.py --num-trial 50 --dafny-path $(which dafny) --model gpt-4o
```

#### Debug Mode
```bash
cd clover
python debug_exp.py --num-trial 1 --verbose 2 --model gpt-4o --dafny-path $(which dafny)
```

#### Model Options
By default, the code now uses the `gpt-4o` model. You can specify a different model with the `--model` parameter:
```bash
python exps.py --model gpt-4-1106-preview --dafny-path $(which dafny)
```

Will receive 87% acceptance rate on CloverBench (Ground-Truth) dataset with gpt-4-1106-preview in March 2024.

## Citation
```bibtex
@misc{sun2023clover,
      title={Clover: Closed-Loop Verifiable Code Generation}, 
      author={Chuyue Sun and Ying Sheng and Oded Padon and Clark Barrett},
      year={2023},
      eprint={2310.17807},
      archivePrefix={arXiv},
      primaryClass={cs.SE}
}
```

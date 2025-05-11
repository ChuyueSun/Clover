# Clover Environment Setup Guide

## 1. Virtual Environment Setup

A Python virtual environment has been created in the `clover_env` directory with all the required Python dependencies.

### Creating a new virtual environment
If you need to create the environment from scratch:
```bash
# Create a new virtual environment
python3 -m venv clover_env

# Activate the environment
source clover_env/bin/activate

# Install required packages
pip install "sglang[openai]" tqdm
```

### Using an existing environment
To activate an existing environment:
```bash
source clover_env/bin/activate
```

## 2. Installing Dafny

Dafny is required to run the experiments. You need to install it before running the code:

### macOS (using Homebrew)
```bash
brew install dafny
```

### Linux

#### Ubuntu/Debian
```bash
# Install required dependencies
sudo apt update
sudo apt install -y mono-complete dotnet-sdk-7.0

# Download and install Dafny
VERSION=4.3.0  # Replace with the latest version if needed
wget https://github.com/dafny-lang/dafny/releases/download/v${VERSION}/dafny-${VERSION}-x64-ubuntu-20.04.zip
unzip dafny-${VERSION}-x64-ubuntu-20.04.zip -d dafny
sudo mv dafny /usr/local/
echo 'export PATH="$PATH:/usr/local/dafny"' >> ~/.bashrc
echo 'export PATH="$PATH:/usr/local/dafny"' >> ~/.zshrc
source ~/.bashrc  # Or source ~/.zshrc if using zsh
```

#### Arch Linux
```bash
# Using AUR
yay -S dafny

# Or with paru
paru -S dafny
```

#### Other Linux distributions
1. Download Dafny from the official GitHub repository: https://github.com/dafny-lang/dafny/releases
2. Select the appropriate version for your Linux distribution
3. Extract the archive to a location on your machine (e.g., `/usr/local/dafny` or `~/dafny`)
4. Add the Dafny binary directory to your PATH:
```bash
echo 'export PATH="$PATH:/path/to/dafny"' >> ~/.bashrc
source ~/.bashrc
```

### Manual Installation (Any OS)
If the above methods don't work:
1. Download Dafny from the official GitHub repository: https://github.com/dafny-lang/dafny/releases
2. Extract the archive to a location on your machine
3. Add the Dafny binary directory to your PATH

## 3. Troubleshooting Dafny Installation

### Common Linux Issues
1. **Missing dependencies**: Ensure you have Mono and .NET SDK installed:
```bash
# For Ubuntu/Debian
sudo apt install -y mono-complete dotnet-sdk-7.0
```

2. **Permission issues**: Make sure the Dafny executable is executable:
```bash
chmod +x /path/to/dafny/dafny
```

3. **Path issues**: Verify Dafny is in your PATH:
```bash
which dafny
```

4. **Version compatibility**: Some features may require specific Dafny versions. Version 4.x or newer is recommended.

## 4. Running the Experiments

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

### Fixing OpenAI API Issues
If you encounter API errors like `invalid_parameter_combination` with the OpenAI API:

1. Try creating a private key file with the correct API key:
```bash
echo '#!/bin/bash
export OPENAI_API_KEY="YOUR_ACTUAL_API_KEY_HERE"' > private_key.sh
chmod +x private_key.sh
source private_key.sh
```

2. Specific to Linux, ensure your API key doesn't have any unexpected characters or line breaks.

## 5. Notes

- The `--dafny-path` argument must point to your Dafny executable.
- You may adjust the `--num-trial` parameter as needed.
- Your OpenAI API key must be set in the environment variable for the experiments to work.
- On Linux, shell compatibility might vary. If you encounter issues with `$(which dafny)`, provide the full path directly: `--dafny-path /usr/local/dafny/dafny` or wherever you installed it. 
#!/bin/bash

# Set OpenAI API Key
export OPENAI_API_KEY="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"

# Enable LLM cache
export ENABLE_LLM_CACHE=1

# Set cache directory
CACHE_DIR="llm_cache"

# Ensure cache directory exists
mkdir -p $CACHE_DIR/prompts

# VS Code Dafny path
DAFNY_DLL="/home/chuyue/.cursor-server/extensions/dafny-lang.ide-vscode-3.4.4/out/resources/3.10.0/github/dafny/Dafny.dll"

# Parse command line arguments
TEST_NAME="abs"
VERBOSE=1
MODEL="gpt-4o"
DISABLE_CACHE_ARG=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --test)
      TEST_NAME="$2"
      shift 2
      ;;
    --verbose)
      VERBOSE="$2"
      shift 2
      ;;
    --model)
      MODEL="$2" 
      shift 2
      ;;
    --disable-cache)
      export ENABLE_LLM_CACHE=0
      DISABLE_CACHE_ARG="--disable-cache"
      shift
      ;;
    --cache-dir)
      CACHE_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--test NAME] [--verbose LEVEL] [--model MODEL] [--disable-cache] [--cache-dir DIR]"
      exit 1
      ;;
  esac
done

echo "===== Running Clover with LLM Cache ====="
echo "Test name: $TEST_NAME"
echo "Cache: $(if [[ $ENABLE_LLM_CACHE -eq 1 ]]; then echo 'enabled'; else echo 'disabled'; fi)"
echo "Cache directory: $CACHE_DIR"
echo "Verbosity level: $VERBOSE"
echo "Model: $MODEL"
echo "========================================"

# Run the experiment with clover.py which supports caching
cd clover
python clover.py \
  --test-name $TEST_NAME \
  --verbose $VERBOSE \
  --dafny-path "/usr/bin/dotnet $DAFNY_DLL" \
  --cache-dir "../$CACHE_DIR" \
  $DISABLE_CACHE_ARG

# Print cache statistics at the end
echo -e "\n===== Cache Statistics ====="
python -c "import sys; sys.path.append('.'); from llm_cache import LLMCache; cache = LLMCache('../$CACHE_DIR'); cache.inspect_cache_contents()" 
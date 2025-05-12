#!/bin/bash
# Modified wrapper script that runs Clover with caching enabled and pipes output to analyzer

# Default configuration
CACHE_DIR="llm_cache"
ENABLE_CACHE=1
VERBOSE=2  # Increased verbosity for more logging
TEST_NAME="abs"
# Use CloverBench instead of Dafny
DATASET_DIR="dataset/CloverBench"
# API key for OpenAI - Don't hardcode the key here!
API_KEY=${OPENAI_API_KEY:-""}

# VS Code Dafny path - adjust if needed
DAFNY_DLL="/home/chuyue/.cursor-server/extensions/dafny-lang.ide-vscode-3.4.4/out/resources/3.10.0/github/dafny/Dafny.dll"
DAFNY_PATH="/usr/bin/dotnet $DAFNY_DLL"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --test)
      TEST_NAME="$2"
      shift 2
      ;;
    --no-cache)
      ENABLE_CACHE=0
      shift
      ;;
    --cache-dir)
      CACHE_DIR="$2"
      shift 2
      ;;
    --verbose)
      VERBOSE="$2"
      shift 2
      ;;
    --dataset)
      DATASET_DIR="$2"
      shift 2
      ;;
    --api-key)
      API_KEY="$2"
      shift 2
      ;;
    --dummy)
      # Set a dummy mode that doesn't make real API calls
      export ENABLE_LLM_INFERENCE=0
      shift
      ;;
    --help)
      echo "Usage: $0 [options]"
      echo "Options:"
      echo "  --test NAME     Test name to run (default: abs)"
      echo "  --no-cache      Disable caching"
      echo "  --cache-dir DIR Cache directory (default: llm_cache)"
      echo "  --verbose N     Verbosity level (default: 2)"
      echo "  --dataset DIR   Dataset directory (default: dataset/CloverBench)"
      echo "  --api-key KEY   OpenAI API key"
      echo "  --dummy         Run in dummy mode without making API calls"
      echo "  --help          Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Try '$0 --help' for usage information."
      exit 1
      ;;
  esac
done

# Set environment variables
export ENABLE_LLM_CACHE=$ENABLE_CACHE

# Check for API key
if [ -z "$API_KEY" ]; then
  echo "Error: No OpenAI API key provided."
  echo "Please either:"
  echo "  1. Set OPENAI_API_KEY environment variable before running"
  echo "  2. Use the --api-key parameter when running this script"
  exit 1
fi

export OPENAI_API_KEY=$API_KEY

# Make sure cache directory exists
mkdir -p "$CACHE_DIR"

# Create symbolic links from the expected Dafny paths to the actual CloverBench paths
mkdir -p dataset/Dafny/textbook_algo
mkdir -p dataset/Dafny/textbook_algo_unit_tests
mkdir -p dataset/Dafny/textbook_algo_anno

# Check if sample directory exists in CloverBench
if [ ! -d "${DATASET_DIR}/textbook_algo/${TEST_NAME}" ]; then
  echo "Error: Sample ${TEST_NAME} not found in ${DATASET_DIR}/textbook_algo/"
  exit 1
fi

# Create symbolic links for this test
if [ ! -d "dataset/Dafny/textbook_algo/${TEST_NAME}" ]; then
  # Create test-specific symlinks
  ln -sfn "../../../${DATASET_DIR}/textbook_algo/${TEST_NAME}" "dataset/Dafny/textbook_algo/${TEST_NAME}"
  ln -sfn "../../../${DATASET_DIR}/textbook_algo_unit_tests/${TEST_NAME}" "dataset/Dafny/textbook_algo_unit_tests/${TEST_NAME}"
  ln -sfn "../../../${DATASET_DIR}/textbook_algo_anno/${TEST_NAME}" "dataset/Dafny/textbook_algo_anno/${TEST_NAME}"
fi

# Print configuration
echo "===== Running Clover with LLM Cache and Log Analysis ====="
echo "Test name: $TEST_NAME"
echo "Cache: $([ $ENABLE_CACHE -eq 1 ] && echo "enabled" || echo "disabled")"
echo "Cache directory: $CACHE_DIR"
echo "Dataset directory: $DATASET_DIR"
echo "Verbosity level: $VERBOSE"
echo "API key: ${API_KEY:0:8}... (${#API_KEY} chars)"
echo "LLM inference: $([ -z "$ENABLE_LLM_INFERENCE" ] && echo "enabled" || echo "disabled")"
echo "=========================================================="

# Also enable colors for the script output
export PYTHONIOENCODING=utf-8
export FORCE_COLOR=1

# First run, store the output in a temporary file
LOG_FILE=$(mktemp)
echo "Running Clover with log capture to $LOG_FILE..."

# Run Clover with increased logging 
(cd clover && python clover.py \
  --test-name "$TEST_NAME" \
  --verbose "$VERBOSE" \
  --cache-dir "../$CACHE_DIR" \
  --dafny-path "$DAFNY_PATH" \
  $([ $ENABLE_CACHE -eq 0 ] && echo "--disable-cache")) | tee $LOG_FILE

EXIT_CODE=${PIPESTATUS[0]}

if [ $EXIT_CODE -eq 0 ]; then
  echo -e "\n✅ Clover completed successfully"
else
  echo -e "\n❌ Clover failed with exit code $EXIT_CODE"
fi

# Print cache file stats
echo -e "\nCache Files:"
echo "Cache files: $(find $CACHE_DIR -name "*.json" | wc -l)"
echo "Cache size: $(du -sh $CACHE_DIR | cut -f1)"

# Analyze the log file
echo -e "\nAnalyzing cache logs..."
python analyze_cache_logs.py --log-file $LOG_FILE

# Cleanup
rm $LOG_FILE

exit $EXIT_CODE 
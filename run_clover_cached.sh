#!/bin/bash
# Simple wrapper script to run Clover with caching enabled

# Default configuration
CACHE_DIR="llm_cache"
ENABLE_CACHE=1
VERBOSE=1
TEST_NAME="abs"
# Use CloverBench instead of Dafny
DATASET_DIR="dataset/CloverBench"
# API key for OpenAI
API_KEY="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"

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
    --log-file)
      LOG_FILE="$2"
      shift 2
      ;;
    --help)
      echo "Usage: $0 [options]"
      echo "Options:"
      echo "  --test NAME     Test name to run (default: abs)"
      echo "  --no-cache      Disable caching"
      echo "  --cache-dir DIR Cache directory (default: llm_cache)"
      echo "  --verbose N     Verbosity level (default: 1)"
      echo "  --dataset DIR   Dataset directory (default: dataset/CloverBench)"
      echo "  --api-key KEY   OpenAI API key"
      echo "  --dummy         Run in dummy mode without making API calls"
      echo "  --log-file FILE Log output to specified file"
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
export OPENAI_API_KEY=$API_KEY

# Set logging levels to suppress verbose OpenAI logs
export OPENAI_LOG=error       # Set OpenAI logging to error level only
export HTTPX_LOG=error        # Set HTTPX logging to error level only
export HTTPCORE_LOG=error     # Set HTTPCore logging to error level only

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
echo "===== Running Clover with LLM Cache ====="
echo "Test name: $TEST_NAME"
echo "Cache: $([ $ENABLE_CACHE -eq 1 ] && echo "enabled" || echo "disabled")"
echo "Cache directory: $CACHE_DIR"
echo "Dataset directory: $DATASET_DIR"
echo "Verbosity level: $VERBOSE"
echo "API key: ${API_KEY:0:8}... (${#API_KEY} chars)"
echo "LLM inference: $([ -z "$ENABLE_LLM_INFERENCE" ] && echo "enabled" || echo "disabled")"
if [ ! -z "$LOG_FILE" ]; then
  echo "Log file: $LOG_FILE"
fi
echo "========================================"

# Enable standard terminal settings
export PYTHONIOENCODING=utf-8
export PYTHONUNBUFFERED=1  # Disable output buffering

# Set Python log level based on verbosity
if [ $VERBOSE -ge 2 ]; then
  export PYTHONLOGLEVEL=DEBUG
else
  export PYTHONLOGLEVEL=INFO
fi

# Run Clover from the clover directory
cd clover && python -u clover.py \
  --test-name "$TEST_NAME" \
  --verbose "$VERBOSE" \
  --cache-dir "../$CACHE_DIR" \
  --dafny-path "$DAFNY_PATH" \
  $([ ! -z "$LOG_FILE" ] && echo "--log-file \"../$LOG_FILE\"") \
  $([ $ENABLE_CACHE -eq 0 ] && echo "--disable-cache")

EXIT_CODE=$?

# Return to the main directory
cd ..

if [ $EXIT_CODE -eq 0 ]; then
  echo -e "\n✅ Clover completed successfully"
else
  echo -e "\n❌ Clover failed with exit code $EXIT_CODE"
fi

# Show cache stats using the improved cache_util.py
if [ $ENABLE_CACHE -eq 1 ]; then
  echo -e "\n===== Cache Statistics ====="
  python cache_util.py --cache-dir "$CACHE_DIR" summary
  
  # If verbose is 2 or higher, show more detailed cache info
  if [ $VERBOSE -ge 2 ]; then
    echo -e "\n===== Recent Cache Entries ====="
    python cache_util.py --cache-dir "$CACHE_DIR" list --detailed
    
    # Show one of the recent prompts if they exist
    echo -e "\n===== Recent Prompt Example ====="
    python cache_util.py --cache-dir "$CACHE_DIR" prompts --count 1
  fi
fi

exit $EXIT_CODE 
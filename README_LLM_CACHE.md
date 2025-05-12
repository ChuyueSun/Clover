# LLM Cache for Clover

This document describes the LLM caching implementation for the Clover project.

## Overview

The LLM cache system allows Clover to store and reuse responses from language models, reducing API costs and improving performance for repeated queries.

## Key Components

- `llm_cache.py`: Core cache implementation with key generation and storage
- `test_cache_fix.py`: Test script to verify cache functionality

## Recent Fixes

The following issues were addressed in the recent update:

1. **Improved Cache Key Generation**
   - Better extraction of content from SGLang state objects
   - More robust serialization of complex arguments
   - Added method tracking to improve cache uniqueness

2. **Enhanced Storage**
   - Added support for storing both raw and serialized responses
   - Improved handling of direct API calls
   - Better error handling for serialization issues

3. **Diagnostic Tools**
   - Added cache inspection functionality
   - More detailed debug logging
   - Runtime statistics tracking

## Usage

The cache is automatically enabled when running Clover. It can be configured with the following parameters:

```bash
# Enable or disable caching
export ENABLE_LLM_CACHE=1  # enabled (default)
export ENABLE_LLM_CACHE=0  # disabled

# Run Clover with a specific cache directory
./run_clover_cached.sh --test abs --cache-dir llm_cache
```

## Testing

You can verify the cache is working by running the test script:

```bash
python clover/test_cache_fix.py
```

## Cache Structure

- `llm_cache/*.json`: Cache files with LLM responses
- `llm_cache/prompts/*.md`: Stored prompts for reference

## Troubleshooting

If cache issues occur, try:

1. Clearing the cache: `rm -rf llm_cache/*`
2. Running with verbose mode: `--verbose 2`
3. Using the inspect function: `from llm_cache import LLMCache; cache = LLMCache(); cache.inspect_cache_contents()`
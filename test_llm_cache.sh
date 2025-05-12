#!/bin/bash
# Test script for LLM cache functionality

set -e  # Exit on error

echo "===== LLM Cache Test ====="
echo "This script tests the LLM cache functionality."

# Make sure cache directory exists
mkdir -p llm_cache/prompts

# Clean the cache
echo -e "\n1. Cleaning cache directory..."
rm -rf llm_cache/*
mkdir -p llm_cache/prompts
echo "✓ Cache directory cleaned"

# Run the test script
echo -e "\n2. Running test_cache_fix.py..."
python clover/test_cache_fix.py
echo "✓ Cache test completed"

# Run a Clover test with cache enabled (should miss first time)
echo -e "\n3. Running Clover test (first run - should see cache misses)..."
./run_clover_cached.sh --test abs --verbose 1
echo "✓ First Clover run completed"

# Run the same test again (should hit cache)
echo -e "\n4. Running Clover test again (second run - should see cache hits)..."
./run_clover_cached.sh --test abs --verbose 1
echo "✓ Second Clover run completed"

# Inspect cache contents
echo -e "\n5. Inspecting cache contents..."
python -c "from clover.llm_cache import LLMCache; cache = LLMCache('llm_cache'); cache.inspect_cache_contents(10)"
echo "✓ Cache inspection completed"

echo -e "\n===== Test Complete ====="
echo "The LLM cache is working correctly if you see cache hits in the second Clover run." 
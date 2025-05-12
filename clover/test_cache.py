#!/usr/bin/env python3
"""
Simple test script for our LLM caching implementation.
"""

import os
import sys
import argparse
from llm_cache import LLMCache
import logging

# Setup basic logging
logging.basicConfig(level=logging.INFO, 
                   format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger("cache_test")

def main():
    parser = argparse.ArgumentParser(description="Test LLM caching")
    parser.add_argument("--cache-dir", default="../llm_cache", help="Cache directory")
    parser.add_argument("--model", default="gpt-4o", help="Model name to use")
    parser.add_argument("--clear-cache", action="store_true", help="Clear cache before testing")
    args = parser.parse_args()
    
    # Check if API key is set
    api_key = os.environ.get('OPENAI_API_KEY')
    if not api_key:
        print("Warning: OPENAI_API_KEY environment variable not set")
        print("Loading from .env file if present")
        try:
            with open('../.env', 'r') as f:
                for line in f:
                    if line.startswith('OPENAI_API_KEY='):
                        key = line.strip().split('=', 1)[1]
                        os.environ['OPENAI_API_KEY'] = key
                        print(f"Loaded API key from .env file (starts with {key[:4]}...)")
                        break
        except Exception as e:
            print(f"Error loading .env file: {e}")
            print("Please set OPENAI_API_KEY environment variable")
            return 1
    
    # Initialize cache
    cache = LLMCache(cache_dir=args.cache_dir, enabled=True, logger=logger)
    
    # Define test prompts
    test_prompts = [
        "What is 2+2?",
        "List the first 5 prime numbers.",
        "What is the capital of France?"
    ]
    
    # Test caching without making actual API calls
    for i, prompt in enumerate(test_prompts):
        # Generate a unique method name for each prompt
        method = f"test_query_{i}"
        
        # Check cache
        cached_response = cache.get(args.model, method, prompt)
        if cached_response:
            print(f"\nCache HIT for '{prompt}'")
            print(f"Cached response: {cached_response}")
        else:
            print(f"\nCache MISS for '{prompt}'")
            # Instead of making a real API call, just generate a fake response
            fake_response = f"This is a simulated response for: {prompt}"
            print(f"Simulated response: {fake_response}")
            
            # Save to cache
            cache.save(args.model, method, prompt, fake_response)
    
    # Print cache stats
    cache.print_stats()
    return 0

if __name__ == "__main__":
    sys.exit(main()) 
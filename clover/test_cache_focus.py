#!/usr/bin/env python
"""
Focused test script to verify the LLM cache is storing actual responses, not just verification tests.
"""
import os
import sys
import time
import json
from pathlib import Path

# Import the LLM cache
from llm_cache import LLMCache, create_cached_backend

def print_colored(message, color=None):
    """Print colored text with fallback for non-color terminals."""
    colors = {
        "green": "\033[32m",
        "red": "\033[31m",
        "blue": "\033[34m",
        "yellow": "\033[33m",
        "reset": "\033[0m"
    }
    
    if color and color in colors:
        print(f"{colors[color]}{message}{colors['reset']}")
    else:
        print(message)

def test_direct_cache():
    """Test direct caching without SGLang."""
    print("\n===== Testing Direct LLM Cache =====")
    
    # Create a clean cache directory
    cache_dir = "../direct_cache_test"
    os.makedirs(cache_dir, exist_ok=True)
    for f in Path(cache_dir).glob("*.json"):
        f.unlink()
    
    # Create the cache
    cache = LLMCache(cache_dir=cache_dir, enabled=True)
    
    # Create test parameters similar to real LLM requests
    params = {
        "model": "gpt-4o",
        "direct_api": True,
        "kwargs": {
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "What is the capital of France?"}
            ],
            "temperature": 0.7,
            "max_tokens": 100
        }
    }
    
    # Create a mock response
    response = {
        "response": {
            "id": "mock-response-id",
            "choices": [{
                "message": {"content": "The capital of France is Paris."}
            }]
        }
    }
    
    # First check - save to cache
    print("\n1. Saving to cache...")
    cache.save(params, response)
    print_colored("✓ Saved to cache", "green")
    
    # Second check - retrieve from cache
    print("\n2. Retrieving from cache...")
    cached_result = cache.get(params)
    
    if cached_result and "response" in cached_result:
        print_colored(f"✓ Retrieved from cache: {cached_result['response']['id']}", "green")
    else:
        print_colored("✗ Failed to retrieve from cache", "red")
        return False
    
    # Check directory
    print("\n3. Checking cache directory...")
    cache_files = list(Path(cache_dir).glob("*.json"))
    print(f"Found {len(cache_files)} cache files")
    
    if len(cache_files) > 0:
        with open(cache_files[0], "r") as f:
            content = json.load(f)
            if "response" in content:
                print_colored("✓ Cache file contains response data", "green")
            else:
                print_colored("✗ Cache file does not contain response data", "red")
                return False
    
    print_colored("\n✓ Direct cache test passed!", "green")
    return True

def test_sglang_cache():
    """Test caching with SGLang."""
    print("\n===== Testing SGLang LLM Cache =====")
    
    try:
        import sglang as sgl
        from sglang import OpenAI, set_default_backend
    except ImportError:
        print_colored("✗ SGLang not installed. Skipping test.", "yellow")
        return True
    
    # Create a clean cache directory
    cache_dir = "../sglang_cache_test"
    os.makedirs(cache_dir, exist_ok=True)
    for f in Path(cache_dir).glob("*.json"):
        f.unlink()
    
    try:
        # Create a cached backend
        backend = create_cached_backend(
            model="gpt-4o",
            cache_dir=cache_dir,
            enabled=True
        )
        
        set_default_backend(backend)
        
        # Define a simple query function
        @sgl.function
        def simple_query(s, query_text):
            s += sgl.system("You are a helpful assistant.")
            s += sgl.user(f"Answer this: {query_text}")
            s += sgl.assistant(sgl.gen("answer", max_tokens=50))
            return s["answer"]
        
        # First call - should be a cache miss
        print("\n1. Making first call (should be cache miss)...")
        result1 = simple_query("What is the largest planet in our solar system?")
        print(f"Result: {result1}")
        
        # Second call - should be a cache hit
        print("\n2. Making second call (should be cache hit)...")
        result2 = simple_query("What is the largest planet in our solar system?")
        print(f"Result: {result2}")
        
        # Check cache stats
        stats = backend.get_cache_stats()
        print(f"\nCache stats: hits={stats.get('hits', 0)}, misses={stats.get('misses', 0)}")
        
        # Check directory
        print("\n3. Checking cache directory...")
        cache_files = list(Path(cache_dir).glob("*.json"))
        print(f"Found {len(cache_files)} cache files")
        
        if len(cache_files) > 0:
            with open(cache_files[0], "r") as f:
                content = json.load(f)
                # Check for result or simplified key
                if "result" in content or "simplified" in content:
                    print_colored("✓ Cache file contains result data", "green")
                else:
                    print_colored("✗ Cache file does not contain result data", "red")
                    return False
                
        print_colored("\n✓ SGLang cache test passed!", "green")
        return True
    
    except Exception as e:
        print_colored(f"✗ Error in SGLang test: {e}", "red")
        return False

def main():
    """Run all focused cache tests."""
    print("===== Focused LLM Cache Tests =====")
    
    # Test direct caching
    direct_passed = test_direct_cache()
    
    # Test SGLang caching
    sglang_passed = test_sglang_cache()
    
    if direct_passed and sglang_passed:
        print_colored("\n✓ All cache tests passed!", "green")
    else:
        print_colored("\n✗ Some cache tests failed", "red")

if __name__ == "__main__":
    main() 
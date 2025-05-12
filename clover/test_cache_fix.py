#!/usr/bin/env python
"""
Simple test script to verify the cache fix works.
"""
import os
import sys
import time
import shutil
from pathlib import Path

# Import from the current directory
from llm_cache import LLMCache, create_cached_backend
from clover import setup_cache_logging

def main():
    # Define a test cache directory
    cache_dir = "test_fixed_cache"
    
    # Clean up any existing test cache
    if os.path.exists(cache_dir):
        shutil.rmtree(cache_dir)
    os.makedirs(cache_dir, exist_ok=True)
    
    # Setup logging
    logger = setup_cache_logging(verbose=2)
    
    print("===== Testing Cache Fix =====")
    
    # Create a cache instance
    cache = LLMCache(cache_dir=cache_dir, enabled=True, logger=logger)
    
    # Test basic caching
    print("\n1. Testing basic caching")
    params = {
        "model": "gpt-4o",
        "messages": [{"role": "user", "content": "What is the capital of France?"}]
    }
    result = {"content": "Paris is the capital of France."}
    
    cache.save(params, result)
    cache_result = cache.get(params)
    
    if cache_result and cache_result.get("content") == result["content"]:
        print("✅ Basic caching works")
    else:
        print("❌ Basic caching failed")
    
    # Test SGLang-style parameters
    print("\n2. Testing SGLang-style parameters")
    
    # Create a mock SGLang state object
    class MockMessage:
        def __init__(self, role, content):
            self.role = role
            self.content = content
    
    class MockState:
        def __init__(self):
            self.messages = [
                MockMessage("system", "You are a helpful assistant."),
                MockMessage("user", "What is the largest planet?")
            ]
            self.system_message = "You are a helpful assistant."
    
    class MockSGLState:
        def __init__(self):
            self.state = MockState()
    
    # Create params with mock SGLang object
    sgl_params = {
        "model": "gpt-4o",
        "args": (MockSGLState(),),
        "kwargs": {"max_tokens": 100}
    }
    
    sgl_result = {"content": "Jupiter is the largest planet in our solar system."}
    
    cache.save(sgl_params, sgl_result)
    sgl_cache_result = cache.get(sgl_params)
    
    if sgl_cache_result and sgl_cache_result.get("content") == sgl_result["content"]:
        print("✅ SGLang-style caching works")
    else:
        print("❌ SGLang-style caching failed")
    
    # Test direct API style with complex objects
    print("\n3. Testing direct API style")
    
    class MockResponse:
        def __init__(self):
            self.id = "resp_12345"
            self.choices = [MockChoice()]
        
        def model_dump(self):
            return {"id": self.id, "choices": [{"message": {"content": "Neptune is blue."}}]}
    
    class MockChoice:
        def __init__(self):
            self.message = MockMessage("assistant", "Neptune is blue.")
    
    api_params = {
        "model": "gpt-4o",
        "direct_api": True,
        "kwargs": {
            "messages": [{"role": "user", "content": "What color is Neptune?"}],
            "max_tokens": 50
        }
    }
    
    api_result = {"response": MockResponse(), "response_dict": {"id": "resp_12345"}}
    
    cache.save(api_params, api_result)
    api_cache_result = cache.get(api_params)
    
    if api_cache_result and "response_dict" in api_cache_result:
        print("✅ Direct API caching works")
    else:
        print("❌ Direct API caching failed")
    
    # Inspect the cache contents
    print("\n4. Inspecting cache contents")
    cache.inspect_cache_contents()
    
    print("\n===== Cache Fix Test Complete =====")

if __name__ == "__main__":
    main() 
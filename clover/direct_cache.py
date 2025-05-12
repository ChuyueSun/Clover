#!/usr/bin/env python3
"""
Direct caching implementation for SGLang functions.

This module provides a decorator-based caching system for SGLang API calls,
avoiding the need to monkey patch SGLang internals.
"""

import os
import json
import time
import hashlib
from pathlib import Path
from functools import wraps
import logging

# Get the clover logger
logger = logging.getLogger("clover_cache")

class DirectCache:
    """Simple file-based cache for SGLang function results."""
    
    def __init__(self, cache_dir="llm_cache", enabled=True, verbose=False):
        """Initialize the cache.
        
        Args:
            cache_dir: Directory to store cache files
            enabled: Whether caching is enabled
            verbose: Whether to print verbose logs
        """
        self.cache_dir = Path(cache_dir)
        self.enabled = enabled
        self.verbose = verbose
        self.hits = 0
        self.misses = 0
        self.local_hits = 0  # Hits in the current session
        self.local_misses = 0  # Misses in the current session
        
        # Create cache directory if it doesn't exist
        if self.enabled:
            self.cache_dir.mkdir(parents=True, exist_ok=True)
            logger.info(f"Cache directory: {self.cache_dir}")
    
    def get_key(self, func_name, args_str):
        """Generate a cache key based on function name and arguments."""
        key_data = f"{func_name}:{args_str}"
        return hashlib.md5(key_data.encode('utf-8')).hexdigest()
    
    def get(self, key):
        """Get an item from the cache if it exists."""
        if not self.enabled:
            self.local_misses += 1
            self.misses += 1
            logger.debug(f"Cache disabled, treating as miss for key: {key[:8]}")
            return None
            
        cache_file = self.cache_dir / f"{key}.json"
        if not cache_file.exists():
            self.local_misses += 1
            self.misses += 1
            logger.debug(f"Cache miss: {key[:8]}")
            return None
        
        try:
            with open(cache_file, "r") as f:
                data = json.load(f)
            self.local_hits += 1
            self.hits += 1
            logger.debug(f"Cache hit: {key[:8]}")
            return data.get("result")
        except Exception as e:
            self.local_misses += 1
            self.misses += 1
            logger.error(f"Cache error reading {key[:8]}: {e}")
            return None
    
    def save(self, key, result):
        """Save a result to the cache."""
        if not self.enabled:
            logger.debug(f"Cache disabled, not saving key: {key[:8]}")
            return
            
        cache_file = self.cache_dir / f"{key}.json"
        
        try:
            # Handle non-serializable objects
            if isinstance(result, object) and not isinstance(result, (str, int, float, bool, list, dict, tuple, type(None))):
                result = str(result)
                
            data = {
                "timestamp": time.time(),
                "result": result
            }
            
            with open(cache_file, "w") as f:
                json.dump(data, f, indent=2)
                
            logger.debug(f"Cache save: {key[:8]}")
        except Exception as e:
            logger.error(f"Cache save error for {key[:8]}: {e}")
    
    def get_cache_stats(self):
        """Get cache statistics."""
        return {
            "hits": self.hits,
            "misses": self.misses,
            "local_hits": self.local_hits,
            "local_misses": self.local_misses
        }
        
    def print_stats(self):
        """Print cache statistics."""
        stats = self.get_cache_stats()
        hits = stats["hits"]
        misses = stats["misses"]
        local_hits = stats["local_hits"]
        local_misses = stats["local_misses"]
        
        total = hits + misses
        local_total = local_hits + local_misses
        
        hit_rate = (hits / total * 100) if total > 0 else 0
        local_hit_rate = (local_hits / local_total * 100) if local_total > 0 else 0
        
        logger.info("\nCache Statistics:")
        logger.info(f"  Total hits: {hits}")
        logger.info(f"  Total misses: {misses}")
        if total > 0:
            logger.info(f"  Hit rate: {hit_rate:.1f}%")
        
        logger.info("\nSession Statistics:")
        logger.info(f"  Session hits: {local_hits}")
        logger.info(f"  Session misses: {local_misses}")
        if local_total > 0:
            logger.info(f"  Session hit rate: {local_hit_rate:.1f}%")

# Global cache instance
_cache = None

def setup_direct_cache(cache_dir="llm_cache", enabled=True, verbose=False):
    """Set up the global cache instance."""
    global _cache
    
    # Check if caching is enabled via environment variable
    env_enabled = os.environ.get("ENABLE_LLM_CACHE", "1").lower() in ("1", "true", "yes")
    enabled = enabled and env_enabled
    
    _cache = DirectCache(cache_dir=cache_dir, enabled=enabled, verbose=verbose)
    return _cache

def cached_function(func):
    """Decorator to cache SGLang function results."""
    @wraps(func)
    def wrapper(*args, **kwargs):
        global _cache
        
        # Initialize cache if needed
        if _cache is None:
            _cache = setup_direct_cache()
        
        # Create a string representation of the arguments
        # Skip the first argument (the State object) as it's not serializable
        args_str = str(args[1:]) + str(sorted(kwargs.items()))
        
        # Generate cache key
        key = _cache.get_key(func.__name__, args_str)
        
        # Try to get from cache
        cached_result = _cache.get(key)
        if cached_result is not None:
            logger.debug(f"Returning cached result for {func.__name__}")
            return cached_result
        
        # Cache miss - run function
        logger.debug(f"Cache miss, executing {func.__name__}")
        start_time = time.time()
        result = func(*args, **kwargs)
        elapsed = time.time() - start_time
        logger.debug(f"Function {func.__name__} completed in {elapsed:.2f}s")
        
        # Save to cache
        _cache.save(key, result)
        
        return result
    return wrapper

# Example of how to use this with SGLang functions
if __name__ == "__main__":
    import sglang as sgl
    from sglang import OpenAI, assistant, gen, set_default_backend, system, user
    
    # Set up cache
    cache = setup_direct_cache(cache_dir="test_cache", verbose=True)
    
    # Create backend
    backend = OpenAI(model_name="gpt-4o")
    set_default_backend(backend)
    
    # Define a cached function
    @cached_function
    def generate_response(prompt):
        """Generate a response using SGLang."""
        s = sgl.State()
        s += system("You are a helpful assistant.")
        s += user(prompt)
        s += assistant(gen("response", max_tokens=50))
        return s["response"]
    
    # Test the function
    prompt = "Tell me a fact about caching"
    
    # First call - should be a cache miss
    print("\nFirst call:")
    response1 = generate_response(prompt)
    print(f"Response: {response1}")
    
    # Second call - should be a cache hit
    print("\nSecond call:")
    response2 = generate_response(prompt)
    print(f"Response: {response2}")
    
    # Print stats
    cache.print_stats() 
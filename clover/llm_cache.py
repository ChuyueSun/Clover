"""
A simple caching mechanism for LLM API calls to avoid redundant API calls.
"""

import hashlib
import json
import os
import time
from pathlib import Path
import logging
import colorama
from colorama import Fore, Style

# Initialize colorama
colorama.init(autoreset=True)

# Define color constants for different log types
COLOR_HIT = Fore.GREEN
COLOR_MISS = Fore.RED
COLOR_WRITE = Fore.BLUE
COLOR_ERROR = Fore.YELLOW
COLOR_INFO = Fore.WHITE

# Use a consistent logger name
logger = logging.getLogger("clover_cache")

class LLMCache:
    """
    A class to cache LLM API call results to disk.
    """

    def __init__(
        self,
        cache_dir: str = "llm_cache",
        enabled: bool = True,
        max_age_days: int = 7,
        always_write: bool = False,
        logger=None,
    ):
        """
        Initialize the cache.

        Args:
            cache_dir: Directory to store cache files
            enabled: Whether caching is enabled for reading
            max_age_days: Maximum age of cache entries in days
            always_write: Whether to always write to cache even if reading is disabled
            logger: Optional logger for cache operations
        """
        self.cache_dir = Path(cache_dir)
        self.always_write = always_write

        # Check environment variable to determine if caching is enabled
        enable_cache_env = os.environ.get("ENABLE_LLM_CACHE", "1")
        self.enabled = enabled and enable_cache_env == "1"

        # Log the cache status
        if logger:
            if self.enabled:
                logger.info(
                    f"{COLOR_INFO}LLM cache enabled for reading and writing (from env: ENABLE_LLM_CACHE={enable_cache_env})"
                )
            elif self.always_write:
                logger.info(
                    f"{COLOR_INFO}LLM cache disabled for reading but enabled for writing (from env: ENABLE_LLM_CACHE={enable_cache_env})"
                )
            else:
                logger.info(
                    f"{COLOR_INFO}LLM cache disabled (from env: ENABLE_LLM_CACHE={enable_cache_env})"
                )

        self.max_age_seconds = max_age_days * 24 * 60 * 60
        self.logger = logger or logging.getLogger("clover_cache")

        # Create cache directory if needed for writing
        if self.enabled or self.always_write:
            try:
                self.cache_dir.mkdir(exist_ok=True, parents=True)
                
                # Create prompts directory
                (self.cache_dir / "prompts").mkdir(exist_ok=True, parents=True)
                self.logger.debug(f"Cache directory: {self.cache_dir}")
            except Exception as e:
                self.logger.warning(f"{COLOR_ERROR}Failed to create cache directory: {e}")

        # Cache hit statistics 
        self.hits = 0
        self.misses = 0
        
    def _get_cache_key(self, model, method, prompt, params=None):
        """Generate a unique cache key based on model, method, and prompt."""
        # Create a serializable dict of key parameters
        # Normalize params to ensure consistent key generation
        normalized_params = {}
        
        if params:
            # Copy all parameters except system prompt and feedback text 
            # which will be handled separately
            for k, v in params.items():
                # Skip large text fields from the key to avoid key length issues
                # but include their hashes instead
                if k == 'system' and isinstance(v, str) and len(v) > 100:
                    normalized_params['system_hash'] = hashlib.md5(v.encode()).hexdigest()[:8]
                elif k == 'feedback_prompt' and isinstance(v, str):
                    normalized_params['feedback_hash'] = hashlib.md5(v.encode()).hexdigest()[:8]
                # Always include the iteration and trial_id in the key
                elif k in ['iteration', 'trial_id', 'feedback_hash']:
                    normalized_params[k] = v
        
        # Create the key data with normalized parameters
        key_data = {
            "model": model,
            "method": method,
            "prompt": prompt,
            "params": normalized_params
        }
        
        # Serialize to JSON and hash
        serialized = json.dumps(key_data, sort_keys=True)
        key_hash = hashlib.md5(serialized.encode()).hexdigest()
        
        return key_hash

    def get(self, model, method, prompt, params=None):
        """
        Retrieve a cached result if it exists and is not too old.

        Args:
            model: The model name
            method: The method/function being called (e.g., gen_doc_from_body)
            prompt: The prompt text
            params: Additional parameters that affect the result

        Returns:
            Cached response if cache hit, None if cache miss
        """
        if not self.enabled:
            self.misses += 1
            return None

        try:
            cache_key = self._get_cache_key(model, method, prompt, params)
            cache_file = self.cache_dir / f"{cache_key}.json"

            if not cache_file.exists():
                self.logger.info(f"{COLOR_MISS}Cache MISS: key={cache_key}, method={method}")
                print(f"\nCACHE MISS: method={method}")
                self.misses += 1
                return None

            # Read the cache file
            try:
                with open(cache_file, 'r') as f:
                    cache_data = json.load(f)
            except (json.JSONDecodeError, PermissionError, OSError) as e:
                self.logger.warning(f"{COLOR_ERROR}Cache read error for {cache_key}: {e}")
                self.misses += 1
                return None

            # Check if cache is too old
            timestamp = cache_data.get("timestamp", 0)
            current_time = time.time()

            if current_time - timestamp > self.max_age_seconds:
                self.logger.info(f"{COLOR_MISS}Cache MISS (expired): key={cache_key}, age={(current_time - timestamp) / 86400:.1f} days")
                self.misses += 1
                return None

            # Check that important metadata matches to prevent hash collisions
            if cache_data.get("method") != method or cache_data.get("model") != model:
                self.logger.warning(f"{COLOR_ERROR}Cache metadata mismatch: expected {method}/{model}, got {cache_data.get('method')}/{cache_data.get('model')}")
                self.misses += 1
                return None
                
            # Log more details for debugging
            stored_params = cache_data.get("params", {})
            iteration = stored_params.get("iteration", "?")
            trial_id = stored_params.get("trial_id", "?")
            has_feedback = "feedback_prompt" in stored_params or "feedback_hash" in stored_params
            
            # Format the description for logging
            desc = []
            if "trial_id" in stored_params:
                desc.append(f"trial={trial_id}")
            if "iteration" in stored_params:
                desc.append(f"iter={iteration}")
            if has_feedback:
                desc.append("with_feedback")
                
            desc_str = ", ".join(desc) if desc else "no params"
            
            self.logger.info(f"{COLOR_HIT}Cache HIT: key={cache_key}, method={method}, {desc_str}, age={(current_time - timestamp) / 86400:.1f} days")
            print(f"\nCACHE HIT: method={method}, {desc_str}")
            self.hits += 1
            
            return cache_data.get("response")

        except Exception as e:
            self.logger.warning(f"{COLOR_ERROR}Unexpected error in cache.get: {e}")
            self.misses += 1
            return None

    def save(self, model, method, prompt, response, params=None):
        """
        Save a response to the cache.
        
        Args:
            model: The model name
            method: The method/function being called (e.g., gen_doc_from_body)
            prompt: The prompt text
            response: The response to cache
            params: Additional parameters that affect the result
        """
        # Only skip saving if both enabled and always_write are False
        if not self.enabled and not self.always_write:
            return

        try:
            cache_key = self._get_cache_key(model, method, prompt, params)
            cache_file = self.cache_dir / f"{cache_key}.json"

            # Create a copy of params to avoid modifying the original
            params_copy = {}
            if params:
                params_copy.update(params)
            
            # If feedback text is very large, truncate it for storage efficiency
            if 'feedback_prompt' in params_copy and isinstance(params_copy['feedback_prompt'], str) and len(params_copy['feedback_prompt']) > 2000:
                # Store the full hash but truncate the actual text
                if 'feedback_hash' not in params_copy:
                    params_copy['feedback_hash'] = hashlib.md5(params_copy['feedback_prompt'].encode()).hexdigest()
                params_copy['feedback_prompt'] = params_copy['feedback_prompt'][:2000] + "... [truncated]"

            # Cache data with timestamp and metadata
            cache_data = {
                "timestamp": time.time(),
                "model": model,
                "method": method,
                "prompt": prompt,
                "params": params_copy,
                "response": response
            }

            # Write to the cache file
            if not self.cache_dir.exists():
                self.cache_dir.mkdir(exist_ok=True, parents=True)

            with open(cache_file, "w") as f:
                json.dump(cache_data, f, indent=2)
            
            # Also save a human-readable prompt file
            try:
                # Create a method-specific subdirectory to organize prompts
                method_dir = self.cache_dir / "prompts" / method
                method_dir.mkdir(exist_ok=True, parents=True)
                
                # Create a more descriptive filename
                desc_parts = []
                param_info = params or {}
                
                if 'trial_id' in param_info:
                    desc_parts.append(f"trial{param_info['trial_id']}")
                    
                if 'iteration' in param_info:
                    desc_parts.append(f"iter{param_info['iteration']}")
                    
                if 'feedback_hash' in param_info:
                    desc_parts.append(f"fb{param_info['feedback_hash'][:4]}")
                
                desc_str = "_".join(desc_parts) if desc_parts else "base"
                prompt_file = method_dir / f"{desc_str}_{cache_key[:8]}.md"
                
                with open(prompt_file, "w") as f:
                    f.write(f"# Prompt for {method}\n\n")
                    f.write(f"## Model: {model}\n")
                    f.write(f"## Timestamp: {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
                    f.write(f"## Cache Key: {cache_key}\n\n")
                    
                    # Add metadata about trial and iteration
                    param_info = params or {}
                    
                    # Add feedback information if present in a highlighted section
                    has_feedback = False
                    
                    if 'feedback_prompt' in param_info:
                        has_feedback = True
                        feedback = param_info['feedback_prompt']
                        f.write(f"\n## Feedback to LLM:\n```diff\n+ {feedback.replace('\n', '\n+ ')}\n```\n\n")
                        
                    if 'iteration' in param_info:
                        f.write(f"## Iteration: {param_info['iteration']}")
                        if 'feedback_hash' in param_info:
                            f.write(f" (feedback hash: {param_info['feedback_hash']})")
                        f.write("\n")
                        
                    if 'trial_id' in param_info:
                        f.write(f"## Trial: {param_info['trial_id']}\n")
                    
                    # Show context - what's the sequence of this prompt
                    if has_feedback:
                        f.write("\n## Context:\n")
                        f.write(f"This is iteration {param_info.get('iteration', '?')} with feedback from a previous attempt.\n\n")
                    elif param_info.get('iteration', 0) > 0:
                        f.write("\n## Context:\n")
                        f.write(f"This is iteration {param_info.get('iteration', '?')} but using a cached response.\n\n")
                    
                    f.write(f"## Prompt:\n{prompt}\n\n")
                    
                    # Include parameters but filter out large items like the feedback prompt which is already shown separately
                    filtered_params = {k: v for k, v in param_info.items() if k != 'feedback_prompt'}
                    param_json = "{}" if not filtered_params else json.dumps(filtered_params, indent=2)
                    f.write(f"## Parameters:\n```json\n{param_json}\n```\n\n")
                    
                    f.write(f"## Response:\n{response}\n")
                    
            except Exception as e:
                self.logger.warning(f"{COLOR_ERROR}Failed to save prompt file: {e}")
            
            # Log the operation
            file_size = os.path.getsize(cache_file)
            size_readable = f"{file_size / 1024:.1f}KB" if file_size > 1024 else f"{file_size}B"
            
            # Format description for logging
            desc = []
            if params:
                if "trial_id" in params:
                    desc.append(f"trial={params['trial_id']}")
                if "iteration" in params:
                    desc.append(f"iter={params['iteration']}")
                if "feedback_prompt" in params or "feedback_hash" in params:
                    desc.append("with_feedback")
            
            desc_str = ", ".join(desc) if desc else ""
            
            if self.enabled:
                log_msg = f"{COLOR_WRITE}Cache WRITE: key={cache_key}, method={method}, {desc_str}, size={size_readable}"
                self.logger.info(log_msg)
                print(f"\nCACHE WRITE: method={method}, {desc_str}")
            else:
                self.logger.info(f"{COLOR_WRITE}Cache WRITE (write-only mode): key={cache_key}, method={method}, {desc_str}, size={size_readable}")
                print(f"\nCACHE WRITE (write-only mode): method={method}, {desc_str}")

        except Exception as e:
            self.logger.warning(f"{COLOR_ERROR}Unexpected error in cache.save: {e}")
            import traceback
            self.logger.debug(traceback.format_exc())

    def get_stats(self):
        """Get cache hit statistics."""
        total = self.hits + self.misses
        hit_rate = self.hits / total * 100 if total > 0 else 0
        
        return {
            "hits": self.hits,
            "misses": self.misses,
            "total": total,
            "hit_rate": hit_rate
        }
        
    def print_stats(self):
        """Print cache statistics."""
        stats = self.get_stats()
        hits = stats["hits"]
        misses = stats["misses"]
        total = hits + misses
        
        print(f"\nCache statistics: {hits} hits, {misses} misses")
        if total > 0:
            hit_rate = hits / total * 100
            print(f"Hit rate: {hit_rate:.1f}%")
        else:
            print(f"No cache operations yet") 
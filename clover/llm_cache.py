"""
A simple caching mechanism for LLM API calls to avoid redundant API calls.
"""

import hashlib
import json
import os
import time
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple, Union
import logging
from functools import wraps
import colorama
from colorama import Fore, Style
import sys

# Initialize colorama
colorama.init(autoreset=True)

# Use a consistent logger name but don't configure it yet
# The setup_cache_logging function in clover.py will configure it
logger = logging.getLogger("clover_cache")

# Global cache statistics
_GLOBAL_CACHE_STATS = {
    "hits": 0,
    "misses": 0
}

# Define color constants for different log types
COLOR_HIT = Fore.GREEN
COLOR_MISS = Fore.RED
COLOR_WRITE = Fore.BLUE
COLOR_PROMPT = Fore.CYAN
COLOR_ERROR = Fore.YELLOW
COLOR_INFO = Fore.WHITE
COLOR_DEBUG = Style.DIM + Fore.WHITE

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

        # Check for deprecated environment variable for backward compatibility
        deprecated_cache_env = os.environ.get("LLM_CACHE_ENABLED")
        if deprecated_cache_env is not None:
            if logger:
                logger.warning(
                    f"{COLOR_ERROR}LLM_CACHE_ENABLED is deprecated. Please use ENABLE_LLM_CACHE instead."
                )
            # Still honor the deprecated variable if it's set to disable caching
            if deprecated_cache_env == "0":
                if logger:
                    logger.warning(
                        f"{COLOR_ERROR}Disabling cache due to deprecated LLM_CACHE_ENABLED=0 setting"
                    )
                enable_cache_env = "0"

        # Cache is enabled if passed parameter is True and environment variable is "1"
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
                
                # Verify the directory was created and is writable
                if self.cache_dir.exists():
                    # Create a temporary file to verify write permissions
                    try:
                        test_file = self.cache_dir / ".write_test"
                        test_file.write_text("test")
                        test_file.unlink()
                        self.logger.debug(f"{COLOR_DEBUG}Cache directory verified: {self.cache_dir}")
                    except Exception as e:
                        self.logger.warning(f"{COLOR_ERROR}Cache directory not writable: {e}")
                else:
                    self.logger.warning(f"{COLOR_ERROR}Failed to create cache directory: {self.cache_dir}")
            except Exception as e:
                self.logger.warning(f"{COLOR_ERROR}Failed to create cache directory: {e}")

        # Cache hit statistics - use instance variables but also update global counters
        self.hits = 0
        self.misses = 0
        
        # Create a prompt directory to store prompt files along with responses
        self.prompt_dir = self.cache_dir / "prompts"
        if self.enabled or self.always_write:
            try:
                self.prompt_dir.mkdir(exist_ok=True, parents=True)
            except Exception as e:
                self.logger.warning(f"{COLOR_ERROR}Failed to create prompt directory: {e}")
                
        # Reset global counters when initializing a new cache - comment this to preserve counters across runs
        # global _GLOBAL_CACHE_STATS
        # _GLOBAL_CACHE_STATS["hits"] = 0
        # _GLOBAL_CACHE_STATS["misses"] = 0
        
        # Print current stats at initialization time
        self.print_stats()
    
    def print_stats(self):
        """Print current cache statistics."""
        stats = self.get_stats()
        hits = stats['hits']
        misses = stats['misses']
        print(f"\nCache statistics: {hits} hits, {misses} misses")
        if hits + misses > 0:
            hit_rate = hits / (hits + misses) * 100
            print(f"Hit rate: {hit_rate:.1f}%")
        else:
            print(f"No cache operations yet")

    def _compute_key(self, params: Dict[str, Any]) -> str:
        """Generate a unique cache key based on query parameters dictionary."""
        try:
            # Create a serializable version of params
            serializable_params = {}
            
            # Handle basic parameters first
            if "model" in params:
                serializable_params["model"] = str(params.get("model", ""))
            
            if "engine" in params:
                serializable_params["engine"] = str(params.get("engine", ""))
                
            if "instruction" in params:
                serializable_params["instruction"] = params.get("instruction", "")
                
            if "query" in params:
                serializable_params["query"] = params.get("query", "")
                
            if "max_tokens" in params:
                serializable_params["max_tokens"] = params.get("max_tokens", 0)
                
            if "system_info" in params:
                serializable_params["system_info"] = params.get("system_info", "")
                
            if "exemplars" in params:
                serializable_params["exemplars"] = self._make_serializable(params.get("exemplars", []))
            
            # Special handling for test_key (used in verification tests)
            if "test_key" in params:
                serializable_params["test_key"] = params.get("test_key", "")
            
            # Handle trial number for trial-aware caching
            serializable_params["trial_num"] = params.get("trial_num", 0)
            
            # Handle direct API flag 
            if "direct_api" in params:
                serializable_params["direct_api"] = params.get("direct_api", False)
            
            # Process complex objects: args
            args = params.get("args", ())
            if args:
                try:
                    # Extract key content from args to ensure better cache keys
                    if len(args) > 0:
                        # For SGLang calls, we need better extraction of the content
                        first_arg = args[0]
                        
                        # Check if the first arg has a state attribute (SGLang State object)
                        if hasattr(first_arg, 'state'):
                            state_dict = {}
                            # Extract messages from state
                            if hasattr(first_arg.state, 'messages'):
                                messages = first_arg.state.messages
                                state_dict['messages'] = []
                                for msg in messages:
                                    if hasattr(msg, 'role') and hasattr(msg, 'content'):
                                        state_dict['messages'].append({
                                            'role': msg.role,
                                            'content': msg.content
                                        })
                                    else:
                                        state_dict['messages'].append(str(msg))
                            
                            # Extract any other important state attributes
                            if hasattr(first_arg.state, 'system_message'):
                                state_dict['system_message'] = str(first_arg.state.system_message)
                                
                            serializable_params["state"] = state_dict
                        else:
                            # If it's not a state object, serialize the whole thing
                            serializable_params["args_content"] = self._make_serializable(first_arg)
                    else:
                        serializable_params["args"] = self._make_serializable(args)
                except Exception as e:
                    # Fallback serialization if extraction fails
                    self.logger.debug(f"{COLOR_DEBUG}Error extracting from args: {e}, falling back to simple serialization")
                    serializable_params["args"] = self._make_serializable(args)
            
            # Process complex objects: kwargs
            kwargs = params.get("kwargs", {})
            if kwargs:
                # Check for 'messages' which is a common key for API calls
                if "messages" in kwargs:
                    messages = kwargs["messages"]
                    if isinstance(messages, list) and len(messages) > 0:
                        # Extract message content for better cache keys
                        serializable_params["messages"] = []
                        for msg in messages:
                            if isinstance(msg, dict):
                                # Only keep essential fields for caching
                                serializable_params["messages"].append({
                                    "role": msg.get("role", ""),
                                    "content": msg.get("content", "")
                                })
                # Also check for other common parameters
                for key in ["temperature", "max_tokens", "top_p", "frequency_penalty", "presence_penalty"]:
                    if key in kwargs:
                        serializable_params[key] = kwargs[key]
                
                # Default serialization for remaining kwargs
                serializable_params["kwargs"] = self._make_serializable(kwargs)
            
            # Create a stable string representation and hash
            param_str = json.dumps(serializable_params, sort_keys=True)
            model_str = serializable_params.get('model', 'unknown')
            self.logger.debug(f"{COLOR_DEBUG}Computing cache key for model: {model_str}")
            self.logger.debug(f"{COLOR_DEBUG}Serialized params: {param_str[:200]}...")
            
            # Generate the hash
            cache_key = hashlib.md5(param_str.encode()).hexdigest()
            
            # Save the prompt if this is a direct API call
            if params.get("direct_api", False) and (self.enabled or self.always_write):
                self._save_prompt(cache_key, params)
                
            return cache_key
        except Exception as e:
            self.logger.warning(f"{COLOR_ERROR}Error computing cache key: {e}")
            # Return a fallback key - unlikely to match existing keys
            return f"error_key_{time.time()}"
    
    def _save_prompt(self, cache_key: str, params: Dict[str, Any]) -> None:
        """Save the prompt to a file for future reference."""
        try:
            # Make sure the prompt directory exists
            if not self.prompt_dir.exists():
                self.prompt_dir.mkdir(exist_ok=True, parents=True)
                
            # Create the prompt file
            prompt_file = self.prompt_dir / f"{cache_key}.md"
            
            # Format the prompt components
            prompt_content = "# Cached Prompt\n\n"
            
            # Add model information
            if "model" in params:
                prompt_content += f"## Model\n{params.get('model')}\n\n"
            elif "engine" in params:
                prompt_content += f"## Engine\n{params.get('engine')}\n\n"
                
            # Add timestamp
            prompt_content += f"## Timestamp\n{time.strftime('%Y-%m-%d %H:%M:%S')}\n\n"
                
            # Add system info if available
            system_info = params.get("system_info", "")
            if system_info:
                prompt_content += f"## System\n{system_info}\n\n"
                
            # Add instruction if available
            instruction = params.get("instruction", "")
            if instruction:
                prompt_content += f"## Instruction\n{instruction}\n\n"
                
            # Add exemplars if available
            exemplars = params.get("exemplars", [])
            if exemplars:
                prompt_content += f"## Exemplars\n```json\n{json.dumps(exemplars, indent=2)}\n```\n\n"
                
            # Add query if available
            query = params.get("query", "")
            if query:
                prompt_content += f"## Query\n{query}\n\n"
                
            # Add kwargs if available for API calls
            kwargs = params.get("kwargs", {})
            if kwargs:
                # Extract messages if this is a chat completion
                messages = kwargs.get("messages", [])
                if messages:
                    prompt_content += "## Messages\n"
                    for msg in messages:
                        role = msg.get("role", "")
                        content = msg.get("content", "")
                        prompt_content += f"**{role}**: {content}\n\n"
            
            # Write the prompt file
            with open(prompt_file, "w") as f:
                f.write(prompt_content)
                
            # Get file size for logging
            file_size = os.path.getsize(prompt_file)
            size_readable = f"{file_size / 1024:.1f}KB" if file_size > 1024 else f"{file_size}B"
            
            self.logger.info(f"{COLOR_PROMPT}Prompt WRITE: key={cache_key}, size={size_readable}")
            
        except Exception as e:
            self.logger.warning(f"{COLOR_ERROR}Failed to save prompt: {e}")
        
    def _get_cache_file(self, cache_key: str) -> Path:
        """Get the cache file path for a given key."""
        return self.cache_dir / f"{cache_key}.json"

    def get(self, params: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """
        Retrieve a cached result if it exists and is not too old.

        Args:
            params: Dictionary of parameters that uniquely identify the query

        Returns:
            Cached response dictionary if cache hit, None if cache miss
        """
        # Check if caching is disabled
        if os.environ.get("ENABLE_LLM_CACHE", "1") == "0":
            self.logger.info(f"{COLOR_DEBUG}Cache disabled by environment variable")
            self.misses += 1
            global _GLOBAL_CACHE_STATS
            _GLOBAL_CACHE_STATS["misses"] += 1
            print(f"\nCACHE MISS: Cache disabled by environment variable")
            return None

        if not self.enabled:
            self.misses += 1
            _GLOBAL_CACHE_STATS["misses"] += 1
            print(f"\nCACHE MISS: Cache disabled in configuration")
            return None

        try:
            cache_key = self._compute_key(params)
            cache_file = self._get_cache_file(cache_key)

            if not cache_file.exists():
                self.logger.info(f"{COLOR_MISS}Cache MISS: key={cache_key}, file not found")
                # Print to terminal without colors for better compatibility
                print(f"\nCACHE MISS: key={cache_key}, file not found")
                self.misses += 1
                _GLOBAL_CACHE_STATS["misses"] += 1
                return None

            # Read the cache file
            try:
                cache_data = json.loads(cache_file.read_text())
            except (json.JSONDecodeError, PermissionError, OSError) as e:
                self.logger.warning(f"{COLOR_ERROR}Cache read error for {cache_key}: {e}")
                self.misses += 1
                _GLOBAL_CACHE_STATS["misses"] += 1
                print(f"\nCACHE MISS: Read error for key={cache_key}: {e}")
                return None

            # Check if cache is too old
            timestamp = cache_data.get("timestamp", 0)
            current_time = time.time()

            if current_time - timestamp > self.max_age_seconds:
                self.logger.info(f"{COLOR_MISS}Cache MISS (expired): key={cache_key}, age={(current_time - timestamp) / 86400:.1f} days")
                print(f"\nCACHE MISS (expired): key={cache_key}, age={(current_time - timestamp) / 86400:.1f} days")
                self.misses += 1
                _GLOBAL_CACHE_STATS["misses"] += 1
                return None

            self.logger.info(f"{COLOR_HIT}Cache HIT: key={cache_key}, age={(current_time - timestamp) / 86400:.1f} days")
            # Print to terminal without colors for better compatibility
            print(f"\nCACHE HIT: key={cache_key}, age={(current_time - timestamp) / 86400:.1f} days")
            self.hits += 1
            _GLOBAL_CACHE_STATS["hits"] += 1
            
            # Print updated stats
            self.print_stats()
            
            return cache_data

        except Exception as e:
            self.logger.warning(f"{COLOR_ERROR}Unexpected error in cache.get: {e}")
            self.misses += 1
            _GLOBAL_CACHE_STATS["misses"] += 1
            print(f"\nCACHE MISS: Unexpected error: {e}")
            return None

    def save(self, params: Dict[str, Any], result: Dict[str, Any]) -> None:
        """
        Save a response to the cache.
        
        Args:
            params: Dictionary of parameters that uniquely identify the query
            result: The result to cache
        """
        # Check if caching is disabled
        if os.environ.get("ENABLE_LLM_CACHE", "1") == "0" and not self.always_write:
            self.logger.debug(f"{COLOR_DEBUG}Cache save skipped - disabled by environment variable")
            return

        # Only skip saving if both enabled and always_write are False
        if not self.enabled and not self.always_write:
            return

        try:
            cache_key = self._compute_key(params)
            cache_file = self._get_cache_file(cache_key)

            # Add timestamp to the result
            cache_data = {
                "timestamp": time.time(),
                "params": self._make_serializable(params),
            }
            
            # Merge with the provided result after making it serializable
            serializable_result = self._make_serializable(result)
            cache_data.update(serializable_result)

            # Ensure the cache directory exists
            try:
                if not self.cache_dir.exists():
                    self.cache_dir.mkdir(exist_ok=True, parents=True)
            except Exception as e:
                self.logger.warning(f"{COLOR_ERROR}Failed to create cache directory during save: {e}")
                return

            # Write to the cache file
            try:
                with open(cache_file, "w") as f:
                    json.dump(cache_data, f, indent=2)
                
                # Get file size for logging
                file_size = os.path.getsize(cache_file)
                size_readable = f"{file_size / 1024:.1f}KB" if file_size > 1024 else f"{file_size}B"
                
                if self.enabled:
                    self.logger.info(f"{COLOR_WRITE}Cache WRITE: key={cache_key}, size={size_readable}")
                    # Print to terminal without colors for better compatibility
                    print(f"\nCACHE WRITE: key={cache_key}, size={size_readable}")
                else:
                    self.logger.info(f"{COLOR_WRITE}Cache WRITE (write-only mode): key={cache_key}, size={size_readable}")
                    # Print to terminal without colors for better compatibility
                    print(f"\nCACHE WRITE (write-only mode): key={cache_key}, size={size_readable}")
                    
            except (PermissionError, OSError) as e:
                self.logger.warning(f"{COLOR_ERROR}Cache write error for {cache_key}: {e}")
                return

        except Exception as e:
            self.logger.warning(f"{COLOR_ERROR}Unexpected error in cache.save: {e}")

    def clear(self, max_age_days: Optional[int] = None) -> int:
        """
        Clear old cache entries.

        Args:
            max_age_days: Override the instance max_age_days

        Returns:
            Number of entries cleared
        """
        if not self.enabled or not self.cache_dir.exists():
            return 0

        max_age = (
            max_age_days * 24 * 60 * 60
            if max_age_days is not None
            else self.max_age_seconds
        )
        current_time = time.time()
        cleared_count = 0

        for cache_file in self.cache_dir.glob("*.json"):
            try:
                try:
                    cache_data = json.loads(cache_file.read_text())
                    timestamp = cache_data.get("timestamp", 0)
                except (json.JSONDecodeError, PermissionError, OSError):
                    # Invalid cache file, just delete it
                    cache_file.unlink(missing_ok=True)
                    cleared_count += 1
                    continue

                if current_time - timestamp > max_age:
                    cache_file.unlink(missing_ok=True)
                    cleared_count += 1

            except Exception as e:
                self.logger.warning(f"{COLOR_ERROR}Error clearing cache file {cache_file}: {e}")

        if cleared_count > 0:
            self.logger.info(f"{COLOR_INFO}Cleared {cleared_count} cache entries")
            print(f"\nCleared {cleared_count} cache entries")

        return cleared_count

    def get_stats(self) -> Dict[str, Any]:
        """Get cache hit statistics."""
        # Include both instance and global statistics
        global _GLOBAL_CACHE_STATS
        
        # Calculate local statistics
        local_hits = self.hits
        local_misses = self.misses
        local_total = local_hits + local_misses
        local_hit_rate = local_hits / local_total if local_total > 0 else 0
        
        # Calculate global statistics
        global_hits = _GLOBAL_CACHE_STATS["hits"]
        global_misses = _GLOBAL_CACHE_STATS["misses"]
        global_total = global_hits + global_misses
        global_hit_rate = global_hits / global_total if global_total > 0 else 0
        
        # Return combined stats with both local and global
        return {
            "hits": global_hits,  # Use global stats as the primary
            "misses": global_misses,
            "total": global_total,
            "hit_rate": global_hit_rate,
            "local_hits": local_hits,
            "local_misses": local_misses,
            "local_total": local_total,
            "local_hit_rate": local_hit_rate,
        }

    def _make_serializable(self, obj: Any) -> Any:
        """Convert objects to JSON serializable format."""
        if isinstance(obj, dict):
            return {k: self._make_serializable(v) for k, v in obj.items()}
        elif isinstance(obj, list):
            return [self._make_serializable(item) for item in obj]
        elif isinstance(obj, tuple):
            return [self._make_serializable(item) for item in obj]
        elif hasattr(obj, 'model_dump'):
            try:
                return obj.model_dump()
            except:
                return str(obj)
        elif hasattr(obj, 'to_dict'):
            try:
                return obj.to_dict()
            except:
                return str(obj)
        elif hasattr(obj, '__dict__'):
            try:
                return self._make_serializable(obj.__dict__)
            except:
                return str(obj)
        else:
            try:
                # Check if json.dumps can handle it directly
                json.dumps(obj)
                return obj
            except:
                # Convert to string if not serializable
                return str(obj)

    def inspect_cache_contents(self, limit=5):
        """Print a summary of the cache contents for debugging.
        
        Args:
            limit: Maximum number of entries to display
        """
        if not self.cache_dir.exists():
            print(f"Cache directory does not exist: {self.cache_dir}")
            return
            
        # List cache files
        cache_files = list(self.cache_dir.glob("*.json"))
        if not cache_files:
            print(f"No cache files found in {self.cache_dir}")
            return
            
        print(f"\nCache Directory: {self.cache_dir}")
        print(f"Total cache entries: {len(cache_files)}")
        
        # Show sample entries
        print(f"\nSample cache entries (up to {limit}):")
        for i, cache_file in enumerate(sorted(cache_files, key=lambda f: f.stat().st_mtime, reverse=True)):
            if i >= limit:
                break
                
            try:
                # Read and parse the cache file
                cache_data = json.loads(cache_file.read_text())
                
                # Extract key information
                timestamp = cache_data.get("timestamp", 0)
                params = cache_data.get("params", {})
                
                # Format timestamp
                time_str = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(timestamp))
                
                # Extract model info
                model = params.get("model", "unknown")
                
                # Look for message content
                message_content = ""
                if "messages" in params:
                    messages = params.get("messages", [])
                    if messages and len(messages) > 0:
                        first_msg = messages[0]
                        if isinstance(first_msg, dict) and "content" in first_msg:
                            message_content = first_msg.get("content", "")[:50]
                
                # Print entry summary
                print(f"  • {cache_file.name}")
                print(f"    - Time: {time_str}")
                print(f"    - Model: {model}")
                if message_content:
                    print(f"    - Content: {message_content}...")
                print()
                
            except Exception as e:
                print(f"  • {cache_file.name} (Error: {e})")
                
        # Show prompt files
        if self.prompt_dir.exists():
            prompt_files = list(self.prompt_dir.glob("*.md"))
            print(f"\nPrompt files: {len(prompt_files)}")
        
        return

# Global cache instance for sglang integration
_GLOBAL_CACHE = None

def get_global_cache(cache_dir: str = "llm_cache", enabled: bool = True) -> LLMCache:
    """Get or create a global cache instance."""
    global _GLOBAL_CACHE
    if _GLOBAL_CACHE is None:
        _GLOBAL_CACHE = LLMCache(cache_dir=cache_dir, enabled=enabled)
    return _GLOBAL_CACHE

def monkey_patch_sglang(custom_logger=None):
    """
    Monkey patch sglang to add caching.
    Apply this once at the beginning of your script.
    
    Args:
        custom_logger: Optional logger to use (defaults to clover_cache logger)
    """
    # Use the specified logger or fall back to the global one
    log = custom_logger or logger
    
    try:
        import sglang
        
        # First try to use sgl.OpenAI class directly (based on the implementation provided)
        OpenAIClass = None
        
        # Try to get the OpenAI class from sglang
        try:
            from sglang import OpenAI
            OpenAIClass = OpenAI
            log.info(f"{COLOR_INFO}Using sglang.OpenAI for monkey patching")
        except ImportError:
            pass

        
        # If we couldn't find OpenAI class, we can't monkey patch
        if OpenAIClass is None:
            log.warning(f"{COLOR_ERROR}Could not locate OpenAI class in sglang")
            return None
        
        # For sgl.OpenAI, find the appropriate method to patch
        patch_method = None
        
        # Check for different potential method names to patch
        if hasattr(OpenAIClass, 'generate'):
            patch_method = 'generate'
        elif hasattr(OpenAIClass, 'run'):
            patch_method = 'run'
        elif hasattr(OpenAIClass, 'create'):
            patch_method = 'create'
        elif hasattr(OpenAIClass, '__call__'):
            patch_method = '__call__'
            
        if patch_method is None:
            log.warning(f"{COLOR_ERROR}Could not find a suitable method to patch in {OpenAIClass.__name__}")
            return None
            
        log.info(f"{COLOR_INFO}Will patch {OpenAIClass.__name__}.{patch_method} method")
            
        # Store the original method
        original_method = getattr(OpenAIClass, patch_method)
        
        # Get the global cache
        cache = get_global_cache()
        
        # Store the current trial count
        trial_counter = {
            'current_trial': 0,
            'max_trials': 1
        }
        
        # Function to update trial information
        def update_trial_info(num_trials=1):
            trial_counter['max_trials'] = num_trials
            trial_counter['current_trial'] = 0
            
        # Add the function to sglang for external access
        sglang.update_trial_info = update_trial_info
        
        # Define a wrapper function
        @wraps(original_method)
        def cached_method(self, *args, **kwargs):
            try:
                # Track which trial we're on
                trial_counter['current_trial'] += 1
                if trial_counter['current_trial'] > trial_counter['max_trials']:
                    trial_counter['current_trial'] = 1
                    
                # Get the model name for logging
                model_name = getattr(self, 'model_name', getattr(self, 'model', 'unknown'))
                log.debug(f"{COLOR_DEBUG}SGL method call for model: {model_name}")
                
                # Create a better cache key from arguments
                params = {
                    "model": model_name,
                    "args": args,  # Store exact args without conversion
                    "kwargs": kwargs,  # Store exact kwargs without conversion
                    "trial_num": trial_counter['current_trial']
                }
                
                # Add function name to improve key uniqueness
                if hasattr(original_method, "__qualname__"):
                    params["method"] = original_method.__qualname__
                
                # Try to get from cache
                log.debug(f"{COLOR_DEBUG}Looking up cache key for sglang call")
                cache_result = cache.get(params)
                if cache_result is not None:
                    log.info(f"{COLOR_HIT}SGL Cache HIT: model={model_name}")
                    # Note: cache.get already updates the hit counter
                    return cache_result.get("result")
                
                # Cache miss - call the original method
                log.info(f"{COLOR_MISS}SGL Cache MISS: model={model_name}")
                # Note: cache.get already updates the miss counter
                result = original_method(self, *args, **kwargs)
                
                # Cache the result 
                try:
                    # Make sure we're caching the result properly
                    cache_data = {"result": result}
                    
                    # For debugging purposes, also store a simplified version if possible
                    try:
                        if hasattr(result, 'state') and hasattr(result.state, 'messages'):
                            simplified = {"messages": []}
                            for msg in result.state.messages:
                                if hasattr(msg, 'role') and hasattr(msg, 'content'):
                                    simplified["messages"].append({
                                        "role": msg.role,
                                        "content": msg.content
                                    })
                            cache_data["simplified"] = simplified
                    except Exception:
                        pass  # Ignore simplification errors
                    
                    cache.save(params, cache_data)
                    log.info(f"{COLOR_WRITE}SGL Cache WRITE: model={model_name}")
                except Exception as save_err:
                    log.error(f"{COLOR_ERROR}Failed to save to cache: {save_err}")
                
                return result
            except Exception as e:
                log.error(f"{COLOR_ERROR}Error in cached_method: {e}")
                # Fall back to original method to prevent breaking the application
                return original_method(self, *args, **kwargs)
        
        # Apply the monkey patch
        setattr(OpenAIClass, patch_method, cached_method)
        
        # Expose the cache for access
        OpenAIClass._cache = cache
        
        # For testing, make the cache accessible
        sglang.cache_stats = lambda: cache.get_stats()
        
        log.info(f"{COLOR_INFO}Successfully monkey patched sglang.{OpenAIClass.__name__}")
        return cache
    except ImportError as e:
        log.warning(f"{COLOR_ERROR}Failed to monkey patch sglang: {e}")
        return None
    except Exception as e:
        log.warning(f"{COLOR_ERROR}Error in monkey_patch_sglang: {e}")
        return None

def create_cached_backend(model: str = "gpt-4o", cache_dir: str = "llm_cache", enabled: bool = True, **kwargs):
    """
    Create a backend with caching support.
    
    Args:
        model: The model name
        cache_dir: Directory to store cache files
        enabled: Whether caching is enabled
        **kwargs: Additional keyword arguments, including:
            - num_trials: Number of trials to run (for cache key separation)
            - always_write: Whether to always write to cache even if reading disabled
            - logger: Custom logger to use (defaults to clover_cache logger)
        
    Returns:
        A backend with access to cache statistics
    """
    # Use a consistent logger - either from kwargs or the global one
    log = kwargs.get('logger', logger)
    
    # Create global cache instance regardless of monkey patching success
    always_write = kwargs.get('always_write', False)
    cache = get_global_cache(cache_dir=cache_dir, enabled=enabled)
    
    # Make sure the cache uses the same logger
    cache.logger = log
    
    try:
        # Import sglang here to avoid circular imports
        import sglang as sgl
        from sglang import OpenAI
        
        # Apply monkey patching to sglang
        patched_cache = monkey_patch_sglang(custom_logger=log)
        
        # If monkey patching succeeded, configure it
        if patched_cache is not None:
            log.info(f"{COLOR_INFO}Successfully set up cached backend with monkey patching")
            
            # Set number of trials if provided
            num_trials = kwargs.get('num_trials', 1)
            if hasattr(sgl, 'update_trial_info'):
                sgl.update_trial_info(num_trials)
                log.info(f"{COLOR_INFO}Set trials to {num_trials} for cache key separation")
        else:
            log.warning(f"{COLOR_ERROR}Monkey patching failed, will use direct API calls")
            # In this case, we'll still cache but with direct API approach
        
        # Create the backend
        backend = OpenAI(model)
        
        # Store a reference to the cache for easy access
        backend._cache = cache
        
        # If monkey patching failed, set up direct OpenAI API caching
        if patched_cache is None:
            log.info(f"{COLOR_INFO}Setting up direct OpenAI API caching")
            # Store the original method
            original_create = backend.client.chat.completions.create
            
            # Define wrapped version with caching
            def cached_create(*args, **kwargs):
                try:
                    # Extract key parameters for better cache keys
                    message_content = ""
                    if "messages" in kwargs:
                        messages = kwargs["messages"]
                        for msg in messages:
                            if isinstance(msg, dict) and "content" in msg:
                                message_content += str(msg.get("content", ""))[:100]  # use up to 100 chars per message
                    
                    # Build cache key from parameters
                    params = {
                        "model": model,
                        "direct_api": True,  # Mark as direct API call
                        "message_digest": hashlib.md5(message_content.encode()).hexdigest()[:10],  # Add short digest of content
                        "args": args,  # Store original args
                        "kwargs": kwargs,  # Store original kwargs
                    }
                    
                    # Try to get from cache
                    log.debug(f"{COLOR_DEBUG}Checking cache for direct API call to model: {model}")
                    cache_result = cache.get(params)
                    if cache_result is not None:
                        log.info(f"{COLOR_HIT}Direct API Cache HIT: model={model}")
                        # Note: cache.get already updates the hit counter
                        return cache_result.get("response")
                    
                    # Cache miss, call original method
                    log.info(f"{COLOR_MISS}Direct API Cache MISS: model={model}")
                    # Note: cache.get already updates the miss counter
                    response = original_create(*args, **kwargs)
                    
                    # Save the response with detailed logging
                    log.debug(f"{COLOR_DEBUG}Response type: {type(response)}")
                    
                    # Create a serializable version of the response
                    if hasattr(response, "model_dump"):
                        log.debug(f"{COLOR_DEBUG}Using model_dump for response")
                        response_dict = response.model_dump()
                    elif hasattr(response, "to_dict"):
                        log.debug(f"{COLOR_DEBUG}Using to_dict for response")
                        response_dict = response.to_dict()
                    else:
                        log.debug(f"{COLOR_DEBUG}Using direct response")
                        response_dict = response
                    
                    cache.save(params, {"response": response, "response_dict": response_dict})
                    log.info(f"{COLOR_WRITE}Direct API Cache WRITE: model={model}")
                    
                    return response
                except Exception as e:
                    log.error(f"{COLOR_ERROR}Error in cached_create: {e}")
                    # Fall back to original method
                    return original_create(*args, **kwargs)
            
            # Replace the method
            backend.client.chat.completions.create = cached_create
            log.info(f"{COLOR_INFO}Direct API caching enabled")
        
        # Add a method to get cache stats for backward compatibility
        backend.get_cache_stats = lambda: cache.get_stats()
        
        # Log the creation
        log.info(f"{COLOR_INFO}Created cached backend for model: {model}, cache enabled: {enabled}")
        
        return backend
        
    except Exception as e:
        log.error(f"{COLOR_ERROR}Error creating cached backend: {e}")
        log.warning(f"{COLOR_ERROR}Creating fallback backend without caching")
        
        # Create a fallback backend without caching
        try:
            import sglang as sgl
            from sglang import OpenAI
            backend = OpenAI(model)
            return backend
        except Exception as e2:
            log.error(f"{COLOR_ERROR}Could not create fallback backend: {e2}")
            raise ValueError(f"Failed to create backend: {e} -> {e2}") from e 
import argparse
import logging
from typing import Dict, List
import colorama
from colorama import Fore, Style
import os
from pathlib import Path
import time
import hashlib

# Initialize colorama
colorama.init(autoreset=True)

# Define color constants
COLOR_SUCCESS = Fore.GREEN
COLOR_ERROR = Fore.RED
COLOR_INFO = Fore.BLUE
COLOR_WARN = Fore.YELLOW
COLOR_DEBUG = Style.DIM + Fore.WHITE
COLOR_HIT = Fore.GREEN
COLOR_MISS = Fore.RED

# Flag to control SGLang state printing - set to False to disable
ENABLE_SGLANG_STATE_PRINTING = False

# Disable noisy OpenAI and HTTP client logging
logging.getLogger("openai").setLevel(logging.WARNING)
logging.getLogger("openai._base_client").setLevel(logging.WARNING)
logging.getLogger("httpx").setLevel(logging.WARNING)
logging.getLogger("httpcore").setLevel(logging.WARNING)

# Create logger but don't configure it yet - setup_cache_logging will do that
logger = logging.getLogger("clover_cache")

# Debug function to print SGLang state
def print_sglang_state(s, tag=""):
    """
    Print out the entire SGLang conversation state for debugging.
    
    Args:
        s: SGLang state object
        tag: Optional tag to identify this state dump
    """
    if not ENABLE_SGLANG_STATE_PRINTING:
        return
        
    logger.info(f"{COLOR_DEBUG}==== SGLang State Dump {tag} ====")
    
    try:
        # Try to access messages through the messages() method
        try:
            messages = s.messages()
            logger.info(f"{COLOR_DEBUG}Found {len(messages)} messages using s.messages()")
            for i, m in enumerate(messages):
                role = m.get("role", "unknown")
                content = m.get("content", "")
                role_color = COLOR_INFO if role == "user" else COLOR_SUCCESS if role == "assistant" else COLOR_DEBUG
                logger.info(f"{COLOR_DEBUG}[{i}] {role_color}{role}{COLOR_DEBUG}: {content}")
            return
        except (AttributeError, TypeError) as e:
            logger.debug(f"Could not access messages via s.messages(): {e}")
        
        # If the above fails, try to directly print the prompt components
        # This is a fallback approach for debugging
        logger.info(f"{COLOR_DEBUG}Attempting to print raw state contents:")
        prompt_parts = []
        
        # Print system messages if they exist
        if hasattr(s, '_system') and s._system:
            logger.info(f"{COLOR_DEBUG}SYSTEM: {s._system}")
            prompt_parts.append(f"SYSTEM: {s._system}")
        
        # Print user/assistant message history if it exists
        if hasattr(s, '_role_common'):
            for i, (role, content) in enumerate(s._role_common):
                role_color = COLOR_INFO if role == "user" else COLOR_SUCCESS if role == "assistant" else COLOR_DEBUG
                logger.info(f"{COLOR_DEBUG}[{i}] {role_color}{role}{COLOR_DEBUG}: {content}")
                prompt_parts.append(f"{role.upper()}: {content}")
        
        # If we couldn't log the messages directly, show what we can find
        if not prompt_parts:
            logger.info(f"{COLOR_DEBUG}Could not access messages directly. Dumping object attributes:")
            for attr in dir(s):
                if not attr.startswith('__') and not callable(getattr(s, attr)):
                    value = getattr(s, attr)
                    try:
                        # Limit output size for large attributes
                        if isinstance(value, str) and len(value) > 100:
                            value = value[:100] + "..."
                        logger.info(f"{COLOR_DEBUG}  {attr}: {value}")
                    except:
                        logger.info(f"{COLOR_DEBUG}  {attr}: <error displaying value>")
                    
        logger.info(f"{COLOR_DEBUG}==== End State Dump ====")
    except Exception as e:
        logger.error(f"{COLOR_ERROR}Error printing SGLang state: {e}")
        import traceback
        logger.error(traceback.format_exc())

# Setup logging function
def setup_cache_logging(log_level=logging.INFO, log_file=None, console_output=True, verbose=0):
    """Set up logging for the cache."""
    # Set log level based on verbosity
    if verbose >= 2:
        log_level = logging.DEBUG
    elif verbose >= 1:
        log_level = logging.INFO
    else:
        log_level = logging.WARNING
    
    # Create handlers
    handlers = []
    
    # Add console handler if requested
    if console_output:
        console_handler = logging.StreamHandler()
        # ALWAYS set console handler to DEBUG level to show cache operations
        # This ensures cache hit/miss/write messages are always displayed
        console_handler.setLevel(logging.DEBUG)
        console_handler.setFormatter(logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s'))
        handlers.append(console_handler)
    
    # Add file handler if provided
    if log_file:
        # Ensure directory exists
        log_path = Path(log_file)
        if not log_path.parent.exists():
            log_path.parent.mkdir(parents=True, exist_ok=True)
            
        file_handler = logging.FileHandler(log_file)
        file_handler.setLevel(log_level)
        file_handler.setFormatter(logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s'))
        handlers.append(file_handler)
    
    # Configure logger
    logger.setLevel(logging.DEBUG)  # Always set main logger to DEBUG level
    
    # Remove any existing handlers to avoid duplicates
    for handler in logger.handlers[:]:
        logger.removeHandler(handler)
    
    # Also remove handlers from the root logger to avoid duplicates
    for handler in logging.root.handlers[:]:
        logging.root.removeHandler(handler)
    
    # Add the new handlers
    for handler in handlers:
        logger.addHandler(handler)
    
    # Also disable OpenAI debug logging again to be sure
    logging.getLogger("openai").setLevel(logging.WARNING)
    logging.getLogger("openai._base_client").setLevel(logging.WARNING)
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("httpcore").setLevel(logging.WARNING)
    
    logger.info(f"{COLOR_INFO}Cache logging configured: level={logging.getLevelName(log_level)}, file={log_file}")
    return logger

def print_cache_stats(backend, verbose=0):
    """Print cache statistics."""
    if not hasattr(backend, 'get_cache_stats'):
        print(f"{COLOR_ERROR}Backend does not have cache statistics")
        logger.warning(f"{COLOR_ERROR}Backend does not have cache statistics")
        return
    
    stats = backend.get_cache_stats()
    hits = stats.get('hits', 0)
    misses = stats.get('misses', 0)
    total = hits + misses
    
    # Basic statistics
    if hits > 0:
        print(f"\nCache Statistics:")
        logger.info(f"{COLOR_SUCCESS}Cache Statistics:")
    else:
        print(f"\nCache Statistics:")
        logger.info(f"{COLOR_INFO}Cache Statistics:")
    
    print(f"  • Hits: {hits}")
    print(f"  • Misses: {misses}")
    print(f"  • Total requests: {total}")
    
    if total > 0:
        hit_rate = hits / total * 100
        # Use different colors in log but plain text in terminal output
        color = COLOR_SUCCESS if hit_rate > 50 else COLOR_WARN if hit_rate > 20 else COLOR_ERROR
        print(f"  • Hit rate: {hit_rate:.1f}%")
        logger.info(f"Cache hit rate: {hit_rate:.1f}%")
    
    # Additional statistics for higher verbosity
    if verbose >= 1 and 'local_hits' in stats:
        print(f"\nInstance Statistics:")
        local_hits = stats.get('local_hits', 0)
        local_misses = stats.get('local_misses', 0)
        local_total = local_hits + local_misses
        
        print(f"  • Instance hits: {local_hits}")
        print(f"  • Instance misses: {local_misses}")
        
        if local_total > 0:
            local_hit_rate = local_hits / local_total * 100
            print(f"  • Instance hit rate: {local_hit_rate:.1f}%")

import sglang as sgl
from sglang import OpenAI, assistant, set_default_backend, system, user
from sglang import gen

# Import direct cache
from direct_cache import setup_direct_cache

# Import our simplified cache
from llm_cache import LLMCache

# Import other modules
from equiv_tests import equiv_test_code, equiv_test_doc, equiv_test_spec
import sys_prompts
from utils import (
    compile_dafny,
    extract_body,
    extract_code_from_llm_output,
    extract_spec,
    get_clover_anno_check_template,
    get_clover_complete_program,
    get_clover_components,
    get_clover_input_sample,
    is_dafny_verified,
    merge_pre_and_body,
    merge_spec_and_body,
    no_compile_error,
    run_dafny,
    stream_print,
)

# Global cache instance
_llm_cache = None
_model_name = "gpt-4o"  # Default model name

def get_cache():
    """Get the global cache instance."""
    global _llm_cache
    return _llm_cache

def get_model_name():
    """Get the current model name safely."""
    global _model_name
    return _model_name

@sgl.function
def gen_doc_from_body(s, body):
    # Add logging for cache debugging
    logger.info(f"{COLOR_INFO}gen_doc_from_body called with body: {body[:50]}...")
    call_id = hashlib.md5(body.encode()).hexdigest()[:8]
    logger.info(f"{COLOR_INFO}Function call ID: {call_id}")
    
    # Get cache
    cache = get_cache()
    
    # Prepare prompt
    system_prompt = sys_prompts.SYS_DAFNY
    user_prompt = sys_prompts.GEN_DOC_FROM_BODY + body
    
    # Check cache first if enabled
    if cache and cache.enabled:
        model_name = get_model_name()
        cached_response = cache.get(model_name, "gen_doc_from_body", user_prompt, {"system": system_prompt})
        if cached_response:
            logger.info(f"{COLOR_SUCCESS}Using cached response for gen_doc_from_body")
            return cached_response
    
    # Add prompts to SGLang state
    s += system(system_prompt)
    s += user(user_prompt)
    
    logger.info(f"{COLOR_INFO}Making LLM call, call ID: {call_id}")
    try:
        # Call the standard gen function
        s += assistant(gen("new_doc", max_tokens=1024))
        result = s["new_doc"]
        logger.info(f"{COLOR_SUCCESS}Successfully generated documentation, call ID: {call_id}")
        
        # Cache the response
        if cache:
            model_name = get_model_name()
            cache.save(model_name, "gen_doc_from_body", user_prompt, result, {"system": system_prompt})
            
        return result
    except Exception as e:
        logger.error(f"{COLOR_ERROR}Error generating documentation: {e}, call ID: {call_id}")
        # Create a fallback for debugging if possible
        if hasattr(s, 'state') and hasattr(s.state, 'vars'):
            logger.error(f"{COLOR_ERROR}Debug state keys: {list(s.state.vars.keys())}")
        raise


@sgl.function
def gen_doc_from_spec(s, spec):
    # Add logging for cache debugging
    logger.info(f"{COLOR_INFO}gen_doc_from_spec called with spec: {spec[:50]}...")
    call_id = hashlib.md5(spec.encode()).hexdigest()[:8]
    logger.info(f"{COLOR_INFO}Function call ID: {call_id}")
    
    # Get cache
    cache = get_cache()
    
    # Prepare prompt
    system_prompt = sys_prompts.SYS_DAFNY
    user_prompt = sys_prompts.GEN_DOC_FROM_SPEC + spec
    
    # Check cache first if enabled
    if cache and cache.enabled:
        model_name = get_model_name()
        cached_response = cache.get(model_name, "gen_doc_from_spec", user_prompt, {"system": system_prompt})
        if cached_response:
            logger.info(f"{COLOR_SUCCESS}Using cached response for gen_doc_from_spec")
            return cached_response
    
    # Add prompts to SGLang state
    s += system(system_prompt)
    s += user(user_prompt)
    
    logger.info(f"{COLOR_INFO}Making LLM call, call ID: {call_id}")
    try:
        s += assistant(gen("new_doc", max_tokens=1024))
        result = s["new_doc"]
        logger.info(f"{COLOR_SUCCESS}Successfully generated documentation, call ID: {call_id}")
        
        # Cache the response
        if cache:
            model_name = get_model_name()
            cache.save(model_name, "gen_doc_from_spec", user_prompt, result, {"system": system_prompt})
            
        return result
    except Exception as e:
        logger.error(f"{COLOR_ERROR}Error generating documentation: {e}, call ID: {call_id}")
        # Create a fallback for debugging if possible
        if hasattr(s, 'state') and hasattr(s.state, 'vars'):
            logger.error(f"{COLOR_ERROR}Debug state keys: {list(s.state.vars.keys())}")
        raise


@sgl.function
def gen_body_from_doc(s, doc, head, input_sample, dafny_path, feedback_turn=3, trial_id=0):
    # Add logging to help debug caching issues
    logger.info(f"{COLOR_INFO}gen_body_from_doc called with doc: {doc[:50]}...")
    
    # Get cache
    cache = get_cache()
    
    # Prepare prompt
    system_prompt = sys_prompts.SYS_DAFNY
    user_prompt = sys_prompts.GEN_BODY_FROM_DOC + doc + "\n" + head
    
    # Add a unique identifier for this call to help with cache tracking
    call_id = hashlib.md5(user_prompt.encode()).hexdigest()[:8]
    logger.info(f"{COLOR_INFO}Function call ID: {call_id}")
    
    # Build the SGLang state
    s += system(system_prompt)
    s += user(user_prompt)
    
    print_sglang_state(s, f"Initial State (trial={trial_id})")
    
    # Store feedback and error messages for each iteration
    feedback_history = []
    
    for i in range(feedback_turn):
        # Create cache parameters including trial_id and iteration for this specific attempt
        cache_params = {"system": system_prompt}
        if trial_id > 0:
            cache_params["trial_id"] = trial_id
        # Always include the iteration number in cache params
        cache_params["iteration"] = i
        # Include accumulated feedback in the cache key for iterations > 0
        if i > 0 and feedback_history:
            cache_params["feedback_hash"] = hashlib.md5("".join(feedback_history).encode()).hexdigest()[:8]
            # Store the complete feedback prompt sent to the LLM - this is critical for caching
            cache_params["feedback_prompt"] = feedback_history[-1]
            
        # Log current iteration
        logger.info(f"{COLOR_INFO}Feedback iteration {i+1}/{feedback_turn}, call ID: {call_id}")
        
        # Check cache for this specific iteration
        if cache and cache.enabled:
            model_name = get_model_name()
            iter_desc = f"trial={trial_id}, iter={i+1}"
            cached_response = cache.get(model_name, "gen_body_from_doc", user_prompt, cache_params)
            if cached_response:
                logger.info(f"{COLOR_SUCCESS}Using cached response for gen_body_from_doc ({iter_desc})")
                body = extract_body(cached_response.strip().split("\n"), False)
                # Verify the cached response compiles
                out = compile_dafny(body, dafny_path)
                if no_compile_error(str(out)):
                    logger.info(f"{COLOR_SUCCESS}Cached response verified successfully ({iter_desc})")
                    s += assistant(body)  # Add to SGLang state
                    return body
                else:
                    logger.warning(f"{COLOR_WARN}Cached response for iteration {i+1} didn't compile, trying next step")
        
        # Make a new LLM call for this iteration
        with s.copy() as tmp:
            logger.info(f"{COLOR_INFO}Making LLM call for iteration {i+1}, call ID: {call_id}")
            
            # Debug - print state before LLM call
            print_sglang_state(tmp, f"Before LLM Call - Iteration {i+1}")
            
            try:
                # Make the call and capture response
                tmp += assistant(gen("new_body", max_tokens=1024))
                body = extract_code_from_llm_output(tmp["new_body"])
                logger.info(f"{COLOR_INFO}Successfully received response for iteration {i+1}, call ID: {call_id}")
            except KeyError as e:
                # This indicates the response was not properly cached or returned
                logger.error(f"{COLOR_ERROR}Failed to extract 'new_body' from response: {e}, call ID: {call_id}")
                # Create a fallback for debugging
                logger.error(f"{COLOR_ERROR}Debug state keys: {list(tmp.state.vars.keys()) if hasattr(tmp, 'state') and hasattr(tmp.state, 'vars') else 'No state vars'}")
                raise
                
        # Process the body
        body = extract_body(body.strip().split("\n"), False)
        s += assistant(body)
        
        # Cache this iteration's response regardless of success
        if cache:
            model_name = get_model_name()
            cache.save(model_name, "gen_body_from_doc", user_prompt, body, cache_params)
            logger.info(f"{COLOR_INFO}Cached response for iteration {i+1} (trial={trial_id})")
        
        # Check if it compiles successfully
        out = compile_dafny(body, dafny_path)
        if no_compile_error(str(out)):
            logger.info(f"{COLOR_SUCCESS}Successful code generation after {i+1} iterations, call ID: {call_id}")
            return body
            
        # If there's an error, continue with feedback
        with s.user():
            error_msg = str(out)
            logger.info(f"{COLOR_WARN}Compilation error in iteration {i+1}, call ID: {call_id}")
            feedback = f"This answer got Dafny compile error:\n{error_msg}\nPlease try again by taking the Dafny compiler feedback."
            
            # Log the actual feedback being sent to the LLM
            logger.info(f"{COLOR_INFO}FEEDBACK TO LLM:\n{feedback}")
            
            s += feedback
            # Store the complete feedback message, not just the error
            feedback_history.append(feedback)
            
            # Debug - print state after adding feedback
            print_sglang_state(s, f"After Adding Feedback - Iteration {i+1}")
    
    # If we've exhausted all feedback turns
    logger.warning(f"{COLOR_WARN}Used all feedback turns without success, call ID: {call_id}")
    return body


@sgl.function
def gen_body_from_spec(s, spec, dafny_path, feedback_turn=3, trial_id=0):
    # Add logging for cache debugging
    logger.info(f"{COLOR_INFO}gen_body_from_spec called with spec: {spec[:50]}...")
    call_id = hashlib.md5(spec.encode()).hexdigest()[:8]
    logger.info(f"{COLOR_INFO}Function call ID: {call_id}")
    
    # Get cache
    cache = get_cache()
    
    # Prepare prompt
    system_prompt = sys_prompts.SYS_DAFNY
    user_prompt = sys_prompts.GEN_BODY_FROM_SPEC + spec
    
    # Build SGLang prompt state
    s += system(system_prompt)
    s += user(user_prompt)
    
    # Debug - print initial state
    print_sglang_state(s, f"Initial State - trial={trial_id}")
    
    # Store feedback and error messages for each iteration
    feedback_history = []
    
    body = ""
    for i in range(feedback_turn):
        # Create cache parameters including trial_id and iteration for this specific attempt
        cache_params = {"system": system_prompt}
        if trial_id > 0:
            cache_params["trial_id"] = trial_id
        # Always include the iteration number in cache params
        cache_params["iteration"] = i
        # Include accumulated feedback in the cache key for iterations > 0
        if i > 0 and feedback_history:
            cache_params["feedback_hash"] = hashlib.md5("".join(feedback_history).encode()).hexdigest()[:8]
            # Store the complete feedback prompt sent to the LLM
            cache_params["feedback_prompt"] = feedback_history[-1]
            
        logger.info(f"{COLOR_INFO}Feedback iteration {i+1}/{feedback_turn}, call ID: {call_id}")
        
        # Check cache for this specific iteration
        if cache and cache.enabled:
            model_name = get_model_name()
            iter_desc = f"trial={trial_id}, iter={i+1}"
            cached_response = cache.get(model_name, "gen_body_from_spec", user_prompt, cache_params)
            if cached_response:
                logger.info(f"{COLOR_SUCCESS}Using cached response for gen_body_from_spec ({iter_desc})")
                body = extract_body(cached_response.strip().split("\n"), False)
                # Verify the cached response compiles
                out = compile_dafny(body, dafny_path)
                if no_compile_error(str(out)):
                    logger.info(f"{COLOR_SUCCESS}Cached response verified successfully ({iter_desc})")
                    s += assistant(body)  # Add to SGLang state
                    return True, body
                else:
                    logger.warning(f"{COLOR_WARN}Cached response for iteration {i+1} didn't compile, trying next step")
        
        # Make a new LLM call for this iteration
        with s.copy() as tmp:
            logger.info(f"{COLOR_INFO}Making LLM call for iteration {i+1}, call ID: {call_id}")
            
            # Debug - print state before LLM call
            print_sglang_state(tmp, f"Before LLM Call - Iteration {i+1}")
            
            try:
                # Use the standard gen function
                tmp += assistant(gen("body", max_tokens=1024))
                body = extract_code_from_llm_output(tmp["body"])
                logger.info(f"{COLOR_INFO}Successfully received response for iteration {i+1}, call ID: {call_id}")
            except Exception as e:
                logger.error(f"{COLOR_ERROR}Error in LLM call: {e}, call ID: {call_id}")
                if hasattr(tmp, 'state') and hasattr(tmp.state, 'vars'):
                    logger.error(f"{COLOR_ERROR}Debug state keys: {list(tmp.state.vars.keys())}")
                raise
                
        body = extract_body(body.strip().split("\n"), False)
        s += assistant(body)
        
        # Cache this iteration's response regardless of success
        if cache:
            model_name = get_model_name()
            cache.save(model_name, "gen_body_from_spec", user_prompt, body, cache_params)
            logger.info(f"{COLOR_INFO}Cached response for iteration {i+1} (trial={trial_id})")
            
        # Check if it compiles successfully
        out = compile_dafny(body, dafny_path)
        if no_compile_error(str(out)):
            logger.info(f"{COLOR_SUCCESS}Successful body generation after {i+1} iterations, call ID: {call_id}")
            return True, body
            
        with s.user():
            error_msg = str(out)
            logger.info(f"{COLOR_WARN}Compilation error in iteration {i+1}, call ID: {call_id}")
            feedback = f"This answer got Dafny compile error:\n{error_msg}\nPlease try again by taking the Dafny compiler feedback."
            
            # Log the actual feedback being sent to the LLM
            logger.info(f"{COLOR_INFO}FEEDBACK TO LLM:\n{feedback}")
            
            s += feedback
            # Store the complete feedback message, not just the error
            feedback_history.append(feedback)
            
            # Debug - print state after adding feedback
            print_sglang_state(s, f"After Adding Feedback - Iteration {i+1}")

    logger.warning(f"{COLOR_WARN}Used all feedback turns without success, call ID: {call_id}")
    return False, body


@sgl.function
def gen_spec_from_doc(s, doc, head, dafny_path, feedback_turn=3, trial_id=0):
    # Add logging for cache debugging
    logger.info(f"{COLOR_INFO}gen_spec_from_doc called with doc: {doc[:50]}...")
    user_prompt = sys_prompts.GEN_SPEC_FROM_DOC + doc + "\n" + head
    call_id = hashlib.md5(user_prompt.encode()).hexdigest()[:8]
    logger.info(f"{COLOR_INFO}Function call ID: {call_id}")
    
    # Get cache
    cache = get_cache()
    
    # Prepare prompt
    system_prompt = sys_prompts.SYS_DAFNY
    
    # Build SGLang prompt state
    s += system(system_prompt)
    s += user(user_prompt)
    
    # Debug - print initial state
    print_sglang_state(s, "Initial State")
    
    # Store feedback and error messages for each iteration
    feedback_history = []
    
    # Track failed response hashes to avoid reusing them
    failed_response_hashes = set()
    
    for i in range(feedback_turn):
        # Create cache parameters including trial_id and iteration for this specific attempt
        cache_params = {"system": system_prompt}
        if trial_id > 0:
            cache_params["trial_id"] = trial_id
        # Always include the iteration number in cache params
        cache_params["iteration"] = i
        # Include accumulated feedback in the cache key for iterations > 0
        if i > 0 and feedback_history:
            cache_params["feedback_hash"] = hashlib.md5("".join(feedback_history).encode()).hexdigest()[:8]
            # Store the complete feedback prompt sent to the LLM
            cache_params["feedback_prompt"] = feedback_history[-1]
            
        logger.info(f"{COLOR_INFO}Feedback iteration {i+1}/{feedback_turn}, call ID: {call_id}")
        
        # Check cache first for this specific iteration
        if cache and cache.enabled:
            model_name = get_model_name()
            iter_desc = f"trial={trial_id}, iter={i+1}"
            cached_response = cache.get(model_name, "gen_spec_from_doc", user_prompt, cache_params)
            if cached_response:
                # Skip failed responses we've already tried
                response_hash = hashlib.md5(cached_response.encode()).hexdigest()
                if response_hash in failed_response_hashes:
                    logger.warning(f"{COLOR_WARN}Skipping previously failed cached response ({iter_desc})")
                else:
                    logger.info(f"{COLOR_SUCCESS}Using cached response for gen_spec_from_doc ({iter_desc})")
                    spec = extract_spec(cached_response.strip().split("\n"), False)
                    # Verify the cached response compiles
                    out = compile_dafny(spec, dafny_path)
                    if no_compile_error(str(out)):
                        logger.info(f"{COLOR_SUCCESS}Cached response verified successfully ({iter_desc})")
                        s += assistant(spec)  # Add to SGLang state
                        return spec
                    else:
                        # Mark this response as failed to avoid reusing it
                        failed_response_hashes.add(response_hash)
                        logger.warning(f"{COLOR_WARN}Cached response for iteration {i+1} didn't compile, trying next step")
                        # Log the error for debugging
                        logger.warning(f"{COLOR_WARN}Compile error: {out}")
        
        # Make a new LLM call for this iteration
        with s.copy() as tmp:
            logger.info(f"{COLOR_INFO}Making LLM call for iteration {i+1}, call ID: {call_id}")
            
            # Debug - print state before LLM call
            print_sglang_state(tmp, f"Before LLM Call - Iteration {i+1}")
            
            try:
                # Use standard gen function
                tmp += assistant(gen("new_spec", max_tokens=2048))
                spec = extract_code_from_llm_output(tmp["new_spec"])
                logger.info(f"{COLOR_INFO}Successfully received response for iteration {i+1}, call ID: {call_id}")
            except Exception as e:
                logger.error(f"{COLOR_ERROR}Error in LLM call: {e}, call ID: {call_id}")
                if hasattr(tmp, 'state') and hasattr(tmp.state, 'vars'):
                    logger.error(f"{COLOR_ERROR}Debug state keys: {list(tmp.state.vars.keys())}")
                raise
                
        spec = extract_spec(spec.strip().split("\n"), False)
        s += assistant(spec)
        
        # Try to fix known ghost predicate issues in the spec
        fixed_spec = spec
        
        # Cache this iteration's response regardless of success
        if cache:
            model_name = get_model_name()
            response_hash = hashlib.md5(spec.encode()).hexdigest()
            
            # Add metadata about compilation status
            cache_params["response_hash"] = response_hash
            
            cache.save(model_name, "gen_spec_from_doc", user_prompt, spec, cache_params)
            logger.info(f"{COLOR_INFO}Cached response for iteration {i+1} (trial={trial_id})")

        # Check if it compiles successfully
        out = compile_dafny(spec, dafny_path)
        if no_compile_error(str(out)):
            logger.info(f"{COLOR_SUCCESS}Successful spec generation after {i+1} iterations, call ID: {call_id}")
            return spec
        else:
            # Mark this response as failed
            failed_response_hashes.add(response_hash)
            
        # Prepare for next iteration with feedback
        with s.user():
            error_msg = str(out)
            logger.info(f"{COLOR_WARN}Compilation error in iteration {i+1}, call ID: {call_id}")
            feedback = f"This answer got Dafny compile error:\n{error_msg}\nPlease try again by taking the Dafny compiler feedback."
            
            # Log the actual feedback being sent to the LLM
            logger.info(f"{COLOR_INFO}FEEDBACK TO LLM:\n{feedback}")
            
            s += feedback
            # Store the complete feedback message, not just the error
            feedback_history.append(feedback)
            
            # Debug - print state after adding feedback
            print_sglang_state(s, f"After Adding Feedback - Iteration {i+1}")

    logger.warning(f"{COLOR_WARN}Used all feedback turns without success, call ID: {call_id}")
    return spec


def doc_to_body_reconstruct(
    doc: str, body: str, input_sample: str, dafny_path, feedback_turn=3, num_trial=1, verbose=0
):
    head = body.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_body_from_doc(
            doc, head, input_sample, dafny_path, feedback_turn=feedback_turn, 
            trial_id=k,  # Pass the trial ID to differentiate cache keys
            stream=(verbose >= 2)
        )
        if verbose >= 2:
            stream_print(s)
        new_body = str(s.ret_value)
        if not equiv_test_code(body, new_body, input_sample, dafny_path, verbose=verbose):
            if verbose >= 2:
                logger.error(f"\n###### Clover Info::Attempt ({k+1}) Doc -> body reconstruction failed.\n")
        else:
            if verbose >= 1:
                logger.info(f"\n###### Clover Info::Attempt ({k+1}) Doc -> body reconstruction succeeded.\n")
            success = True
            break
    if not success and verbose >= 1:
        logger.error(f"\n###### Clover Info::Doc -> body reconstruction failed for {num_trial} attempts.\n")
    return success


def body_to_doc_reconstruct(doc: str, body: str, num_trial=1, verbose=0):
    head = body.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_doc_from_body(body, stream=(verbose >= 2))
        if verbose >= 2:
            stream_print(s)
        new_doc = str(s.ret_value)
        if not equiv_test_doc(doc, new_doc, head, verbose=verbose):
            if verbose >= 2:
                logger.error(f"\n###### Clover Info::Attempt ({k+1}) Body -> doc reconstruction failed.\n")
        else:
            if verbose >= 1:
                logger.info(f"\n###### Clover Info::Attempt ({k+1}) Body -> doc reconstruction succeeded.\n")
            success = True
            break
    if not success and verbose >= 1:
        logger.error(f"\n###### Clover Info::Body -> doc reconstruction failed for {num_trial} attempts.\n")
    return success


def doc_to_spec_reconstruct(
    doc: str, spec: str, anno_check_template: Dict[str, str], dafny_path, num_trial=1, verbose=0
):
    head = spec.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_spec_from_doc(
            doc, head, dafny_path, 
            trial_id=k,  # Pass the trial ID to differentiate cache keys
            stream=(verbose >= 2)
        )
        if verbose >= 2:
            stream_print(s)
        new_spec = str(s.ret_value)
        if not equiv_test_spec(spec, new_spec, anno_check_template, dafny_path, verbose=verbose):
            if verbose >= 2:
                logger.error(f"\n###### Clover Info::Attempt ({k+1}) Doc -> spec reconstruction failed.\n")
        else:
            if verbose >= 1:
                logger.info(f"\n###### Clover Info::Attempt ({k+1}) Doc -> spec reconstruction succeeded.\n")
            success = True
            break
    if not success and verbose >= 1:
        logger.error(f"\n###### Clover Info::Doc -> spec reconstruction failed for {num_trial} attempts.\n")
    return success


def spec_to_doc_reconstruct(doc: str, spec: str, num_trial=1, verbose=0):
    head = spec.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_doc_from_spec(spec, stream=(verbose >= 2))
        if verbose >= 2:
            stream_print(s)
        new_doc = str(s.ret_value)
        if not equiv_test_doc(doc, new_doc, head, verbose=verbose):
            if verbose >= 2:
                logger.error(f"\n###### Clover Info::Attempt ({k+1}) Spec -> doc reconstruction failed.\n")
        else:
            if verbose >= 1:
                logger.info(f"\n###### Clover Info::Attempt ({k+1}) Spec -> doc reconstruction succeeded.\n")
            success = True
            break
    if not success and verbose >= 1:
        logger.error(f"\n###### Clover Info::Spec -> doc reconstruction failed for {num_trial} attempts.\n")
    return success


def spec_soundness(spec: str, body: str, dafny_path, verbose=0):
    body_with_spec = merge_spec_and_body(spec, body)
    out, err = run_dafny(body_with_spec, dafny_path)
    if not is_dafny_verified(str(out)):
        if verbose >= 1:
            logger.error(f"\n###### Clover Info::Dafny verifier failed.\n")
        return False
    if verbose >= 1:
        logger.info(f"\n###### Clover Info::Dafny verifier passed.\n")
    return True


def spec_to_body_reconstruct(
    spec: str, body: str, input_sample: str, dafny_path, feedback_turn=3, num_trial=1, verbose=0
):
    # completeness (spec -> body)
    success = False
    for k in range(num_trial):
        s = gen_body_from_spec(
            spec, dafny_path, 
            feedback_turn=feedback_turn, 
            trial_id=k,  # Pass the trial ID to differentiate cache keys
            stream=(verbose >= 2)
        )
        if verbose >= 2:
            stream_print(s)
        verified, new_body = s.ret_value
        if not verified:
            if verbose >= 2:
                logger.error(f"\n###### Clover Info::Attempt ({k+1}) Failed to reconstruct a body that can be verified.\n")
        elif not equiv_test_code(body, new_body, input_sample, dafny_path, verbose=verbose):
            if verbose >= 2:
                logger.error(f"\n###### Clover Info::Attempt ({k+1}) Spec -> body reconstruction failed.\n")
        else:
            if verbose >= 1:
                logger.info(f"\n###### Clover Info::Attempt ({k+1}) Spec -> body reconstruction succeeded.\n")
            success = True
            break
    if not success and verbose >= 1:
        logger.error(f"\n###### Clover Info::Spec -> body reconstruction failed for {num_trial} attempts.\n")
    return success


def clover(
    program: List[str],
    input_sample,
    anno_check_template,
    dafny_path,
    feedback_turn=3,
    num_trial=1,
    verbose=0,
    early_quit=False,
    just_body=False
):
    doc, spec, body = get_clover_components(program)
    if just_body:
        ret = [None] * 1

        # doc & body consistency
        ret[0] = doc_to_body_reconstruct(
            doc,
            body,
            input_sample,
            dafny_path,
            feedback_turn=feedback_turn,
            num_trial=num_trial,
            verbose=verbose,
        )
        if verbose >= 2:
            logger.info(f"\n###### Final Clover Result: {all(ret)}, {ret}")
        else:
            logger.info(f"\n###### Final Clover Result: {all(ret)}")
            if not all(ret):
                logger.error(f"{COLOR_ERROR}Doc -> body reconstruction failed")

        return all(ret), ret
    else:
        ret = [None] * 6
        test_types = [
            "Doc -> body reconstruction",
            "Body -> doc reconstruction",
            "Doc -> spec reconstruction",
            "Spec -> doc reconstruction",
            "Spec soundness verification",
            "Spec -> body reconstruction"
        ]

        # doc & body consistency
        ret[0] = doc_to_body_reconstruct(
            doc,
            body,
            input_sample,
            dafny_path,
            feedback_turn=feedback_turn,
            num_trial=num_trial,
            verbose=verbose,
        )
        if early_quit and not ret[0]:
            logger.error(f"{COLOR_ERROR}Early quit: Doc -> body reconstruction failed")
            return False, ret
        body_with_pre = merge_pre_and_body(spec, body)
        ret[1] = body_to_doc_reconstruct(
            doc, body_with_pre, num_trial=num_trial, verbose=verbose
        )
        if early_quit and not ret[1]:
            logger.error(f"{COLOR_ERROR}Early quit: Body -> doc reconstruction failed")
            return False, ret

        # doc & spec consistency
        ret[2] = doc_to_spec_reconstruct(
            doc, spec, anno_check_template, dafny_path, num_trial=num_trial, verbose=verbose
        )
        if early_quit and not ret[2]:
            logger.error(f"{COLOR_ERROR}Early quit: Doc -> spec reconstruction failed")
            return False, ret
        ret[3] = spec_to_doc_reconstruct(
            doc, spec, num_trial=num_trial, verbose=verbose)
        if early_quit and not ret[3]:
            logger.error(f"{COLOR_ERROR}Early quit: Spec -> doc reconstruction failed")
            return False, ret

        # spec & body consistency
        ret[4] = spec_soundness(spec, body, dafny_path, verbose=verbose)
        if early_quit and not ret[4]:
            logger.error(f"{COLOR_ERROR}Early quit: Spec soundness verification failed")
            return False, ret
        ret[5] = spec_to_body_reconstruct(
            spec,
            body,
            input_sample,
            dafny_path,
            feedback_turn=feedback_turn,
            num_trial=num_trial,
            verbose=verbose,
        )
        if early_quit and not ret[5]:
            logger.error(f"{COLOR_ERROR}Early quit: Spec -> body reconstruction failed")
            return False, ret
        
        # Log detailed results
        if verbose >= 2:
            logger.info(f"\n###### Final Clover Result: {all(ret)}, {ret}")
        else:
            logger.info(f"\n###### Final Clover Result: {all(ret)}")
            if not all(ret):
                logger.error(f"{COLOR_ERROR}Clover test failed. Test results:")
                for i, (test_result, test_name) in enumerate(zip(ret, test_types)):
                    status = f"{COLOR_SUCCESS}PASS" if test_result else f"{COLOR_ERROR}FAIL"
                    logger.error(f"{i+1}. {status}: {test_name}")
        
        return all(ret), ret


# Main function with updated cache initialization
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--test-name", type=str, default="abs")
    parser.add_argument("--verbose", type=int, default=1)
    parser.add_argument("--early-quit", action="store_true")
    parser.add_argument("--just-body", action="store_true")
    parser.add_argument("--num-trial", type=int, default=1, help="Number of trials to run")
    parser.add_argument("--dafny-path", type=str, required=True)
    parser.add_argument("--cache-dir", type=str, default="../llm_cache")
    parser.add_argument("--disable-cache", action="store_true")
    parser.add_argument("--model", type=str, default="gpt-4o", help="LLM model to use")
    parser.add_argument("--log-file", type=str, help="Log to file")
    args = parser.parse_args()
    
    # Setup logging based on verbosity
    setup_cache_logging(
        log_file=args.log_file,
        verbose=args.verbose
    )
    
    # Initialize cache
    cache_enabled = not args.disable_cache
    
    # Update model name without using the global keyword here
    _model_name = args.model
    logger.info(f"{COLOR_INFO}Using model: {_model_name}")
    
    # Create the global llm_cache instance
    _llm_cache = LLMCache(
        cache_dir=args.cache_dir,
        enabled=cache_enabled,
        max_age_days=7,
        always_write=True,  # Always write to cache even if reading is disabled
        logger=logger
    )
    
    # Create backend
    backend = OpenAI(model_name=args.model)
    
    # Expose the cache on the backend for compatibility
    backend.get_cache_stats = _llm_cache.get_stats
    
    # Set as default backend
    set_default_backend(backend)
    
    # Show cache status
    if cache_enabled and args.verbose >= 1:
        logger.info(f"{COLOR_INFO}LLM caching enabled, using directory: {args.cache_dir}")
    else:
        logger.info(f"{COLOR_INFO}LLM caching disabled for reading (but will still write to cache)")
    
    program_path = (
        f"dataset/CloverBench/textbook_algo/{args.test_name}/{args.test_name}_strong.dfy"
    )
    program_with_pre_path = f"dataset/CloverBench/textbook_algo/{args.test_name}/{args.test_name}_code_with_pre.dfy"
    doc_path = (
        f"dataset/CloverBench/textbook_algo/{args.test_name}/{args.test_name}_spec.txt"
    )
    input_sample_path = f"dataset/CloverBench/textbook_algo_unit_tests/{args.test_name}/{args.test_name}_tests.dfy"
    anno_check_template_path = f"dataset/CloverBench/textbook_algo_anno/{args.test_name}/{args.test_name}_anno_check_template.dfy"

    program = get_clover_complete_program(program_path, doc_path)
    input_sample = get_clover_input_sample(input_sample_path)
    anno_check_template = get_clover_anno_check_template(
        anno_check_template_path)
    
    # Run Clover test
    result, details = clover(
        program,
        input_sample,
        anno_check_template,
        args.dafny_path,
        feedback_turn=3,
        num_trial=args.num_trial,
        verbose=args.verbose,
        early_quit=args.early_quit,
        just_body=args.just_body
    )
    
    # Print result
    if result:
        logger.info(f"{COLOR_SUCCESS}Passed the Clover test? {result}")
    else:
        logger.info(f"{COLOR_ERROR}Passed the Clover test? {result}")
        
        # Print detailed report for failures
        if not args.just_body:
            test_types = [
                "Doc -> body reconstruction",
                "Body -> doc reconstruction",
                "Doc -> spec reconstruction",
                "Spec -> doc reconstruction",
                "Spec soundness verification",
                "Spec -> body reconstruction"
            ]
            
            print("\nDetailed Test Results:")
            print("=====================")
            for i, (test_result, test_name) in enumerate(zip(details, test_types)):
                status = "✓" if test_result else "✗"
                color = COLOR_SUCCESS if test_result else COLOR_ERROR
                print(f"{i+1}. {color}{status} {test_name}{Style.RESET_ALL}")
            print()
    
    # Print cache stats
    if _llm_cache:
        _llm_cache.print_stats()


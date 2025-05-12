import argparse
import logging
from typing import Dict, List
import colorama
from colorama import Fore, Style
import os
from pathlib import Path
import time

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

# Disable noisy OpenAI and HTTP client logging
logging.getLogger("openai").setLevel(logging.WARNING)
logging.getLogger("openai._base_client").setLevel(logging.WARNING)
logging.getLogger("httpx").setLevel(logging.WARNING)
logging.getLogger("httpcore").setLevel(logging.WARNING)

# Create logger but don't configure it yet - setup_cache_logging will do that
logger = logging.getLogger("clover_cache")

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
from sglang import OpenAI, assistant, gen, set_default_backend, system, user

# Import LLM cache
from llm_cache import create_cached_backend

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


@sgl.function
def gen_doc_from_body(s, body):
    s += system(sys_prompts.SYS_DAFNY)
    s += user(sys_prompts.GEN_DOC_FROM_BODY + body)
    s += assistant(gen("new_doc", max_tokens=512))
    return s["new_doc"]


@sgl.function
def gen_doc_from_spec(s, spec):
    s += system(sys_prompts.SYS_DAFNY)
    s += user(sys_prompts.GEN_DOC_FROM_SPEC + spec)
    s += assistant(gen("new_doc", max_tokens=512))
    return s["new_doc"]


@sgl.function
def gen_body_from_doc(s, doc, head, input_sample, dafny_path, feedback_turn=3):
    s += system(sys_prompts.SYS_DAFNY)
    s += user(sys_prompts.GEN_BODY_FROM_DOC + doc + "\n" + head)
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("new_body", max_tokens=1024))
            body = extract_code_from_llm_output(tmp["new_body"])
        body = extract_body(body.strip().split("\n"), False)
        s += assistant(body)
        out = compile_dafny(body, dafny_path)
        if no_compile_error(str(out)):
            return body
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."
    return body


@sgl.function
def gen_body_from_spec(s, spec, dafny_path, feedback_turn=3):
    s += system(sys_prompts.SYS_DAFNY)
    s += user(sys_prompts.GEN_BODY_FROM_SPEC + spec)
    body = ""
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("body", max_tokens=1024))
            body = extract_code_from_llm_output(tmp["body"])
        body = extract_body(body.strip().split("\n"), False)
        s += assistant(body)
        out = compile_dafny(body, dafny_path)
        if no_compile_error(str(out)):
            return True, body
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."

    return False, body


@sgl.function
def gen_spec_from_doc(s, doc, head, dafny_path, feedback_turn=3):
    s += system(sys_prompts.SYS_DAFNY)
    s += user(sys_prompts.GEN_SPEC_FROM_DOC + doc + "\n" + head)
    for i in range(feedback_turn):
        with s.copy() as tmp:
            tmp += assistant(gen("new_spec", max_tokens=512))
            spec = extract_code_from_llm_output(tmp["new_spec"])
        spec = extract_spec(spec.strip().split("\n"), False)
        s += assistant(spec)

        out = compile_dafny(spec, dafny_path)
        if no_compile_error(str(out)):
            return spec
        with s.user():
            s += "This answer got Dafny compile error:\n" + str(out) + "\n"
            s += "Please try again by taking the Dafny compiler feedback."

    return spec


def doc_to_body_reconstruct(
    doc: str, body: str, input_sample: str, dafny_path, feedback_turn=3, num_trial=1, verbose=0
):
    head = body.split("\n")[0]
    success = False
    for k in range(num_trial):
        s = gen_body_from_doc(
            doc, head, input_sample, dafny_path, feedback_turn=feedback_turn, stream=(
                verbose >= 2)
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
        s = gen_spec_from_doc(doc, head, dafny_path, stream=(verbose >= 2))
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
            spec, dafny_path, feedback_turn=feedback_turn, stream=(verbose >= 2))
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

        return all(ret), ret
    else:
        ret = [None] * 6

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
            return False, ret
        body_with_pre = merge_pre_and_body(spec, body)
        ret[1] = body_to_doc_reconstruct(
            doc, body_with_pre, num_trial=num_trial, verbose=verbose
        )
        if early_quit and not ret[1]:
            return False, ret

        # doc & spec consistency
        ret[2] = doc_to_spec_reconstruct(
            doc, spec, anno_check_template, dafny_path, num_trial=num_trial, verbose=verbose
        )
        if early_quit and not ret[2]:
            return False, ret
        ret[3] = spec_to_doc_reconstruct(
            doc, spec, num_trial=num_trial, verbose=verbose)
        if early_quit and not ret[3]:
            return False, ret

        # spec & body consistency
        ret[4] = spec_soundness(spec, body, dafny_path, verbose=verbose)
        if early_quit and not ret[4]:
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
            return False, ret
        if verbose >= 2:
            logger.info(f"\n###### Final Clover Result: {all(ret)}, {ret}")

        return all(ret), ret


# debug purpose
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--test-name", type=str, default="abs")
    parser.add_argument("--verbose", type=int, default=1)
    parser.add_argument("--early-quit", action="store_true")
    parser.add_argument("--just-body", action="store_true")
    parser.add_argument("--dafny-path", type=str, required=True)
    parser.add_argument("--cache-dir", type=str, default="../llm_cache")
    parser.add_argument("--disable-cache", action="store_true")
    parser.add_argument("--log-file", type=str, help="Log to file")
    args = parser.parse_args()
    
    # Setup logging based on verbosity
    setup_cache_logging(
        log_file=args.log_file,
        verbose=args.verbose
    )
    
    # Create a cached backend instead of direct OpenAI
    cache_enabled = not args.disable_cache
    backend = create_cached_backend(
        model="gpt-4-1106-preview",
        cache_dir=args.cache_dir,
        enabled=cache_enabled,
        logger=logger
    )
    set_default_backend(backend)
    
    # Test cache operations to verify everything is working correctly
    if cache_enabled and args.verbose >= 1:
        print("\n======= Verifying Cache Operations =======")
        # Access the cache instance
        cache = None
        if hasattr(backend, '_cache'):
            cache = backend._cache
        else:
            # Create a temporary cache for testing
            from llm_cache import LLMCache
            cache = LLMCache(cache_dir=args.cache_dir, enabled=True, logger=logger)
        
        # Generate a unique test key based on current timestamp
        test_key = f"test-{int(time.time())}"
        
        # Simulate a cache miss and write
        dummy_params = {"model": "gpt-4-1106-preview", "test_key": test_key}
        dummy_result = {"result": "Cache verification test", "timestamp": time.time()}
        
        # Should be a miss since this is a new key
        cache_result = cache.get(dummy_params)
        # Should create a write
        cache.save(dummy_params, dummy_result)
        # Should be a hit now
        cache_result = cache.get(dummy_params)
        
        print("======= Cache Verification Complete =======\n")
    
    if args.verbose >= 1:
        logger.info(f"{COLOR_INFO}LLM cache {'enabled' if cache_enabled else 'disabled'}, using directory: {args.cache_dir}")
    
    program_path = (
        f"../dataset/CloverBench/textbook_algo/{args.test_name}/{args.test_name}_strong.dfy"
    )
    program_with_pre_path = f"../dataset/CloverBench/textbook_algo/{args.test_name}/{args.test_name}_code_with_pre.dfy"
    doc_path = (
        f"../dataset/CloverBench/textbook_algo/{args.test_name}/{args.test_name}_spec.txt"
    )
    input_sample_path = f"../dataset/CloverBench/textbook_algo_unit_tests/{args.test_name}/{args.test_name}_tests.dfy"
    anno_check_template_path = f"../dataset/CloverBench/textbook_algo_anno/{args.test_name}/{args.test_name}_anno_check_template.dfy"

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
        verbose=args.verbose,
        early_quit=args.early_quit,
        just_body=args.just_body
    )
    
    # Print result
    if result:
        logger.info(f"{COLOR_SUCCESS}Passed the Clover test? {result}")
    else:
        logger.info(f"{COLOR_ERROR}Passed the Clover test? {result}")
    
    # Print cache stats with enhanced formatting
    if hasattr(backend, 'get_cache_stats'):
        print_cache_stats(backend, args.verbose)

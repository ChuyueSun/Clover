# Ground Truth Dataset for dafny programs with complete annotations
- `{test_name}`
    - `{test_name}_annotation.dfy`: signature with annotation, no code.
    - `{test_name}_code_with_pre.dfy`: signature with code and asserted preconditions in code, no annotations. Used for code2doc generation.
    - `{test_name}_no_code_spec.dfy`: docstrings with signature.
    - `{test_name}_random.dfy`: randomized function/variable names with no annotation.
    - `{test_name}_spec.txt`: docstring
    - `{test_name}_strong.dfy`: complete dafny programs with annotations
    - `{test_name}_todo.dfy`: signature with code, annotations replaced with the "#TOFILL" string.

SYS_DAFNY = "You are an expert in Dafny. \
You will be given tasks dealing with Dafny programs including precise docstrings and annotations.\n"

GEN_BODY_FROM_SPEC = "Given an empty Dafny program with function head and specifications.\
You are asking to generate the full Dafny code so that it can be verified by Dafny with the given specification. \
Please return the whole program.\
If loop is needed, use while instead of for. \
Do not use helper functions. \
Do not modify the function signature and specifications. Do not explain.\
Below is the function head with specifications:\n"

GEN_DOC_FROM_BODY = "Given a Dafny program. \
Please return an one line docstring of the given dafny code's complete functional behavior. \
Do not mention implementation details. Mention 'assert' as preconditions in the docstring. Describe every detail. \
Please only return the docstring. Do not explain.\
Below is the Dafny program:\n"

GEN_BODY_FROM_DOC = "Given a docstring and the function signature for a Dafny program. \
Please return a Dafny program that implements the functionality described in the docstring. \
If loop is needed, use while instead of for. \
Please only return the Dafny program. Do not explain.\
Below is the docstring and the function signature:\n"

GEN_DOC_FROM_SPEC = "Given the function signature and its specifications for a Dafny program. \
Please return a short and concise docstring of the functional behavior implied by the specifications. \
Do not mention implementation details. \
Please only return the docstring. Do not explain.\
Below is the Dafny signature and its specifications:\n"

GEN_SPEC_FROM_DOC = "Given the function signature and its docstring for a Dafny program. \
Please return the function signature along with specifications include pre- and post- conditions. \
Put one condition in one line. \
Do not return the docstring and the function implementation. Do not use helper functions. Do not explain.\
Below is the docstring and function signature:\n"

DOC_EQUIV = "Determine if two docstrings describe the functional behavior of a dafny program equivalently. \
Make sure the two docstrings can imply each other. \
return YES or NO, and then explain.\n"


if __name__ == "__main__":
    print(SYS_GEN_BODY)
    print(GEN_BODY_FROM_SPEC)

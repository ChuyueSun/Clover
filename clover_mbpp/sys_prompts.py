Grammar_tutorial = r"""Dafny Grammar tutorial: Map Comprehension Expression
Examples:

```dafny
map x : int | 0 <= x <= 10 :: x * x;
map x : int | 0 <= x <= 10 :: -x := x * x;
function square(x : int) : int { x * x }
method test()
{
  var m := map x : int | 0 <= x <= 10 :: x * x;
  ghost var im := imap x : int :: x * x;
  ghost var im2 := imap x : int :: square(x);
}
```

Iterating over the contents of a map uses the component sets: Keys, Values, and Items.
The iteration loop follows the same patterns as for sets:

```dafny
method m<T(==),U(==)> (m: map<T,U>) {
  var items := m.Items;
  while items != {}
    decreases |items|
  {
    var item :| item in items;
    items := items - { item };
    print item.0, " ", item.1, "\n";
  }
}
```

Seq:
for any sequence s of type seq<T>, expression e of type T, integer-based numeric index i satisfying 0 <= i < |s|, and integer-based numeric bounds lo and hi satisfying 0 <= lo <= hi <= |s|, noting that bounds can equal the length of the sequence, sequences support the following operations:

expression	precedence	result type	description
e in s	4	bool	sequence membership
e !in s	4	bool	sequence non-membership
|s|	11	nat	sequence length
s[i]	11	T	sequence selection
s[i := e]	11	seq<T>	sequence update
s[lo..hi]	11	seq<T>	subsequence
s[lo..]	11	seq<T>	drop
s[..hi]	11	seq<T>	take

String:
A special case of a sequence type is seq<char>, 
for which Dafny provides a synonym: string. 
Strings are like other sequences, but provide additional syntax for sequence display expressions, namely string literals

Sortedness: 
Array sorted
```
forall j, k :: 0 <= j < k < a.Length ==> a[j] <= a[k]
```

Array unchanged
```
forall i :: 0 <= i < a.Length ==> a[i] == old(a[i])
```
Dafny Grammar tutorial ends here.
"""

SYS_DAFNY = "You are an expert in Dafny. \
You will be given tasks dealing with Dafny programs including precise docstrings and specifications.\n"

GEN_BODY_FROM_SPEC = Grammar_tutorial + "Given an empty Dafny program with function head and specifications, \
you are asked to generate the full Dafny code so that it can be verified by Dafny with the given specification. \
Please return the whole program. \
If loop is needed, use while instead of for. \
Do not use helper functions. \
Do not modify the function signature and specifications. Directly return the Dafny program without any explanation. \
Below is the function head with specifications:\n"

GEN_DOC_FROM_BODY = "Given a Dafny program. \
Please return an detailed docstring of the given dafny code's complete functional behavior. \
Do not mention implementation details. Mention 'assert' as preconditions in the docstring. Deduce postconditions. \
Describe every detail. Please only return the docstring. Do not explain. \
Below is the Dafny program:\n"

GEN_BODY_FROM_DOC = Grammar_tutorial + "Given a docstring and the function signature for a Dafny program. \
Please return a Dafny program that implements the functionality described in the docstring. \
If loop is needed, use while instead of for. \
Please only return the Dafny program. Do not explain. \
Below is the docstring and the function signature:\n"

GEN_DOC_FROM_SPEC = "Given the function signature and its specifications for a Dafny program. \
Please return a short and concise docstring of the functional behavior implied by the specifications. \
Do not mention implementation details. \
Please only return the docstring. Do not explain. \
Below is the Dafny signature and its specifications:\n"

GEN_SPEC_FROM_DOC = Grammar_tutorial + "Given the function signature and its docstring for a Dafny program. \
Please return the function signature along with specifications include pre- and post- conditions. \
Put one condition in one line. \
Do not return the docstring and the function implementation. Do not use helper functions. \
Do not explain. \
Below is the docstring and function signature:\n"

DOC_EQUIV = "Determine if two docstrings describe the same functional behavior of a dafny program. \
Return YES or NO, and then explain the reason.\n"


if __name__ == "__main__":
    pass

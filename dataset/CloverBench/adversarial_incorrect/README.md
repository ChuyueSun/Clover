Construct incorrect examples. But always keep Dafny verification succeeds.

|  | Code | Annotation | Docstring | Note |
| --- | --- | --- | --- | --- |
| C0  | - | - | - | omitted: ground-truth |
| C1 | - | - | mutated | mutated & strengthen/weaken docstring  |
| C2 | - | mutated | - | weaken annotation |
| C3 | - | mutated | mutated | weaken annotation and docstring simultaneously |
| C4 | mutated | - | - | omitted: cannot pass annotation soundness check  |
| C5 | mutated | - | mutated | omitted: cannot pass annotation soundness check  |
| C6 | mutated | mutated | - | code still satisfies annotation |
| C7 | mutated | mutated | mutated | omitted: non-sense or is a variant of another ground-truth |
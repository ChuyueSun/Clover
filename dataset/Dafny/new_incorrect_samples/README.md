Construct incorrect examples. But always keep Dafny verification succeeds.

|  | Code | Annotation | Docstring | Note |
| --- | --- | --- | --- | --- |
| C0  | - | - | - | omitted: ground-truth |
| C1 | - | - | mutated | original C2 |
| C2 | - | mutated | - | original C1: need to update, weaken annotation only |
| C3 | - | mutated | mutated | new: weaken annotation and docstring |
| C4 | mutated | - | - | omitted: no inconsistency example that can pass Dafny |
| C5 | mutated | - | mutated | omitted: no inconsistency example that can pass Dafny |
| C6 | mutated | mutated | - | original C3 |
| C7 | mutated | mutated | mutated | omitted: non-sense or is a variant of another ground-truth |

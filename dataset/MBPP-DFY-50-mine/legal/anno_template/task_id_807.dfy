predicate pre_original(a: array<int>, found: bool, index: int)
reads a
{
true
}

predicate pre_gen(a: array<int>, found: bool, index: int)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, found: bool, index: int)
  ensures pre_original(a, found, index) <==> pre_gen(a, found, index)
{
}

predicate post_original(a: array<int>, found: bool, index: int)
  requires pre_original(a, found, index)
  reads a
{
true
}

predicate post_gen(a: array<int>, found: bool, index: int)
  requires pre_original(a, found, index)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, found: bool, index: int)
  requires pre_original(a, found, index)
  requires pre_gen(a, found, index)
  ensures post_original(a, found, index) <==> post_gen(a, found, index)
{
}

predicate IsOdd(x: int)
{
  x % 2 != 0
}
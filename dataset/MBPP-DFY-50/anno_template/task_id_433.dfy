predicate pre_original(n: int, a: array<int>, result: bool)
reads a
{
true
}

predicate pre_gen(n: int, a: array<int>, result: bool)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(n: int, a: array<int>, result: bool)
  ensures pre_original(n, a, result) <==> pre_gen(n, a, result)
{
}

predicate post_original(n: int, a: array<int>, result: bool)
  requires pre_original(n, a, result)
  reads a
{
true
}

predicate post_gen(n: int, a: array<int>, result: bool)
  requires pre_original(n, a, result)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(n: int, a: array<int>, result: bool)
  requires pre_original(n, a, result)
  requires pre_gen(n, a, result)
  ensures post_original(n, a, result) <==> post_gen(n, a, result)
{
}
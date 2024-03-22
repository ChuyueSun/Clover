predicate pre_original(a: array<int>, diff: int)
reads a
{
true
}

predicate pre_gen(a: array<int>, diff: int)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, diff: int)
  ensures pre_original(a, diff) <==> pre_gen(a, diff)
{
}

predicate post_original(a: array<int>, diff: int)
  requires pre_original(a, diff)
  reads a
{
true
}

predicate post_gen(a: array<int>, diff: int)
  requires pre_original(a, diff)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, diff: int)
  requires pre_original(a, diff)
  requires pre_gen(a, diff)
  ensures post_original(a, diff) <==> post_gen(a, diff)
{
}
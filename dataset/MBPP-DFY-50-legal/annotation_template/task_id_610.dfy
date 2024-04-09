predicate pre_original(s: array<int>, k: int, v: array<int>)
reads s, v
{
true
}

predicate pre_gen(s: array<int>, k: int, v: array<int>)
reads s, v
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: array<int>, k: int, v: array<int>)
  ensures pre_original(s, k, v) <==> pre_gen(s, k, v)
{
}

predicate post_original(s: array<int>, k: int, v: array<int>)
  requires pre_original(s, k, v)
  reads s, v
{
true
}

predicate post_gen(s: array<int>, k: int, v: array<int>)
  requires pre_original(s, k, v)
  reads s, v
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: array<int>, k: int, v: array<int>)
  requires pre_original(s, k, v)
  requires pre_gen(s, k, v)
  ensures post_original(s, k, v) <==> post_gen(s, k, v)
{
}
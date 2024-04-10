predicate pre_original(a: array<int>, sorted: bool)
reads a
{
true
}

predicate pre_gen(a: array<int>, sorted: bool)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, sorted: bool)
  ensures pre_original(a, sorted) <==> pre_gen(a, sorted)
{
}

predicate post_original(a: array<int>, sorted: bool)
  requires pre_original(a, sorted)
  reads a
{
true
}

predicate post_gen(a: array<int>, sorted: bool)
  requires pre_original(a, sorted)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, sorted: bool)
  requires pre_original(a, sorted)
  requires pre_gen(a, sorted)
  ensures post_original(a, sorted) <==> post_gen(a, sorted)
{
}
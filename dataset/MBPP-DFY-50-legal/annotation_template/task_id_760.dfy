predicate pre_original(a: array<int>, result: bool)
reads a
{
true
}

predicate pre_gen(a: array<int>, result: bool)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, result: bool)
  ensures pre_original(a, result) <==> pre_gen(a, result)
{
}

predicate post_original(a: array<int>, result: bool)
  requires pre_original(a, result)
  reads a
{
true
}

predicate post_gen(a: array<int>, result: bool)
  requires pre_original(a, result)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, result: bool)
  requires pre_original(a, result)
  requires pre_gen(a, result)
  ensures post_original(a, result) <==> post_gen(a, result)
{
}
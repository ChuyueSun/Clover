predicate pre_original(a: array<int>, b: array<int>, result: bool)
reads a,b
{
true
}

predicate pre_gen(a: array<int>, b: array<int>, result: bool)
reads a,b
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, b: array<int>, result: bool)
  ensures pre_original(a, b, result) <==> pre_gen(a, b, result)
{
}

predicate post_original(a: array<int>, b: array<int>, result: bool)
  requires pre_original(a, b, result)
  reads a,b
{
true
}

predicate post_gen(a: array<int>, b: array<int>, result: bool)
  requires pre_original(a, b, result)
  reads a,b
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, b: array<int>, result: bool)
  requires pre_original(a, b, result)
  requires pre_gen(a, b, result)
  ensures post_original(a, b, result) <==> post_gen(a, b, result)
{
}
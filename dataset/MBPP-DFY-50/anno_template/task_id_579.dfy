predicate pre_original(a: array<int>, b: array<int>, result: seq<int>)
reads a, b
{
true
}

predicate pre_gen(a: array<int>, b: array<int>, result: seq<int>)
reads a, b
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, b: array<int>, result: seq<int>)
  ensures pre_original(a, b, result) <==> pre_gen(a, b, result)
{
}

predicate post_original(a: array<int>, b: array<int>, result: seq<int>)
  requires pre_original(a, b, result)
  reads a, b
{
true
}

predicate post_gen(a: array<int>, b: array<int>, result: seq<int>)
  requires pre_original(a, b, result)
  reads a, b
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, b: array<int>, result: seq<int>)
  requires pre_original(a, b, result)
  requires pre_gen(a, b, result)
  ensures post_original(a, b, result) <==> post_gen(a, b, result)
{
}
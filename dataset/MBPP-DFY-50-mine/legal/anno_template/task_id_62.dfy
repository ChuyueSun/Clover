predicate pre_original(s: array<int>, min: int)
reads s
{
true
}

predicate pre_gen(s: array<int>, min: int)
reads s

{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: array<int>, min: int)
  ensures pre_original(s, min) <==> pre_gen(s, min)
{
}

predicate post_original(s: array<int>, min: int)
  requires pre_original(s, min)
  reads s

{
true
}

predicate post_gen(s: array<int>, min: int)
  requires pre_original(s, min)
  reads s

{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: array<int>, min: int)
  requires pre_original(s, min)
  requires pre_gen(s, min)
  ensures post_original(s, min) <==> post_gen(s, min)
{
}
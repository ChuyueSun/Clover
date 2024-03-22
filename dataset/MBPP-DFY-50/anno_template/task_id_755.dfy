predicate pre_original(s: array<int>, secondSmallest: int)
reads s
{
true
}

predicate pre_gen(s: array<int>, secondSmallest: int)
reads s
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: array<int>, secondSmallest: int)
  ensures pre_original(s, secondSmallest) <==> pre_gen(s, secondSmallest)
{
}

predicate post_original(s: array<int>, secondSmallest: int)
  requires pre_original(s, secondSmallest)
  reads s
{
true
}

predicate post_gen(s: array<int>, secondSmallest: int)
  requires pre_original(s, secondSmallest)
  reads s
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: array<int>, secondSmallest: int)
  requires pre_original(s, secondSmallest)
  requires pre_gen(s, secondSmallest)
  ensures post_original(s, secondSmallest) <==> post_gen(s, secondSmallest)
{
}
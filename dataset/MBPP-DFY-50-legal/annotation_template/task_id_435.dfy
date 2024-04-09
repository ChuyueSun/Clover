predicate pre_original(n: int, d: int)
{
true
}

predicate pre_gen(n: int, d: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(n: int, d: int)
  ensures pre_original(n, d) <==> pre_gen(n, d)
{
}

predicate post_original(n: int, d: int)
  requires pre_original(n, d)
{
true
}

predicate post_gen(n: int, d: int)
  requires pre_original(n, d)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(n: int, d: int)
  requires pre_original(n, d)
  requires pre_gen(n, d)
  ensures post_original(n, d) <==> post_gen(n, d)
{
}
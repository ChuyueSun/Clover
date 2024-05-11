predicate pre_original(s: string, count: int)
{
true
}

predicate pre_gen(s: string, count: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: string, count: int)
  ensures pre_original(s, count) <==> pre_gen(s, count)
{
}

predicate post_original(s: string, count: int)
  requires pre_original(s, count)
{
true
}

predicate post_gen(s: string, count: int)
  requires pre_original(s, count)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: string, count: int)
  requires pre_original(s, count)
  requires pre_gen(s, count)
  ensures post_original(s, count) <==> post_gen(s, count)
{
}
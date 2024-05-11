predicate pre_original(s: seq<int>, v: int)
{
true
}

predicate pre_gen(s: seq<int>, v: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: seq<int>, v: int)
  ensures pre_original(s, v) <==> pre_gen(s, v)
{
}

predicate post_original(s: seq<int>, v: int)
  requires pre_original(s, v)
{
true
}

predicate post_gen(s: seq<int>, v: int)
  requires pre_original(s, v)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: seq<int>, v: int)
  requires pre_original(s, v)
  requires pre_gen(s, v)
  ensures post_original(s, v) <==> post_gen(s, v)
{
}
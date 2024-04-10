predicate pre_original(s: string, result: bool)
{
true
}

predicate pre_gen(s: string, result: bool)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: string, result: bool)
  ensures pre_original(s, result) <==> pre_gen(s, result)
{
}

predicate post_original(s: string, result: bool)
  requires pre_original(s, result)
{
true
}

predicate post_gen(s: string, result: bool)
  requires pre_original(s, result)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: string, result: bool)
  requires pre_original(s, result)
  requires pre_gen(s, result)
  ensures post_original(s, result) <==> post_gen(s, result)
{
}
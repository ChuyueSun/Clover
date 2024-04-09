predicate pre_original(n: int, result: bool)
{
true
}

predicate pre_gen(n: int, result: bool)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(n: int, result: bool)
  ensures pre_original(n, result) <==> pre_gen(n, result)
{
}

predicate post_original(n: int, result: bool)
  requires pre_original(n, result)
{
true
}

predicate post_gen(n: int, result: bool)
  requires pre_original(n, result)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(n: int, result: bool)
  requires pre_original(n, result)
  requires pre_gen(n, result)
  ensures post_original(n, result) <==> post_gen(n, result)
{
}
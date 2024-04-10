predicate pre_original(a: int, b: int, c: int, min: int)
{
true
}

predicate pre_gen(a: int, b: int, c: int, min: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: int, b: int, c: int, min: int)
  ensures pre_original(a, b, c, min) <==> pre_gen(a, b, c, min)
{
}

predicate post_original(a: int, b: int, c: int, min: int)
  requires pre_original(a, b, c, min)
{
true
}

predicate post_gen(a: int, b: int, c: int, min: int)
  requires pre_original(a, b, c, min)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: int, b: int, c: int, min: int)
  requires pre_original(a, b, c, min)
  requires pre_gen(a, b, c, min)
  ensures post_original(a, b, c, min) <==> post_gen(a, b, c, min)
{
}
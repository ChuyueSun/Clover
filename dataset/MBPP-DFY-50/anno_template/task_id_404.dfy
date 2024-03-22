predicate pre_original(a: int, b: int, minValue: int)
{
true
}

predicate pre_gen(a: int, b: int, minValue: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: int, b: int, minValue: int)
  ensures pre_original(a, b, minValue) <==> pre_gen(a, b, minValue)
{
}

predicate post_original(a: int, b: int, minValue: int)
  requires pre_original(a, b, minValue)
{
true
}

predicate post_gen(a: int, b: int, minValue: int)
  requires pre_original(a, b, minValue)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: int, b: int, minValue: int)
  requires pre_original(a, b, minValue)
  requires pre_gen(a, b, minValue)
  ensures post_original(a, b, minValue) <==> post_gen(a, b, minValue)
{
}
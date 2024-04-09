predicate pre_original(size: int, area: int)
{
true
}

predicate pre_gen(size: int, area: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(size: int, area: int)
  ensures pre_original(size, area) <==> pre_gen(size, area)
{
}

predicate post_original(size: int, area: int)
  requires pre_original(size, area)
{
true
}

predicate post_gen(size: int, area: int)
  requires pre_original(size, area)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(size: int, area: int)
  requires pre_original(size, area)
  requires pre_gen(size, area)
  ensures post_original(size, area) <==> post_gen(size, area)
{
}
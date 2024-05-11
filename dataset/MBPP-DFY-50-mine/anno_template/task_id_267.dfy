predicate pre_original(n: int, sum: int)
{
true
}

predicate pre_gen(n: int, sum: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(n: int, sum: int)
  ensures pre_original(n, sum) <==> pre_gen(n, sum)
{
}

predicate post_original(n: int, sum: int)
  requires pre_original(n, sum)
{
true
}

predicate post_gen(n: int, sum: int)
  requires pre_original(n, sum)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(n: int, sum: int)
  requires pre_original(n, sum)
  requires pre_gen(n, sum)
  ensures post_original(n, sum) <==> post_gen(n, sum)
{
}
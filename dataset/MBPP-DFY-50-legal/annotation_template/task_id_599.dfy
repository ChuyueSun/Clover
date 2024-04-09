predicate pre_original(n: int, sum: int, average: real)
{
true
}

predicate pre_gen(n: int, sum: int, average: real)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(n: int, sum: int, average: real)
  ensures pre_original(n, sum, average) <==> pre_gen(n, sum, average)
{
}

predicate post_original(n: int, sum: int, average: real)
  requires pre_original(n, sum, average)
{
true
}

predicate post_gen(n: int, sum: int, average: real)
  requires pre_original(n, sum, average)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(n: int, sum: int, average: real)
  requires pre_original(n, sum, average)
  requires pre_gen(n, sum, average)
  ensures post_original(n, sum, average) <==> post_gen(n, sum, average)
{
}
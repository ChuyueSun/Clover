predicate pre_original(lst: seq<int>, product : int)
{
true
}

predicate pre_gen(lst: seq<int>, product : int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(lst: seq<int>, product : int)
  ensures pre_original(lst, product) <==> pre_gen(lst, product)
{
}

predicate post_original(lst: seq<int>, product : int)
  requires pre_original(lst, product)
{
true
}

predicate post_gen(lst: seq<int>, product : int)
  requires pre_original(lst, product)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(lst: seq<int>, product : int)
  requires pre_original(lst, product)
  requires pre_gen(lst, product)
  ensures post_original(lst, product) <==> post_gen(lst, product)
{
}
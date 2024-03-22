predicate pre_original(arr: array<int>, elem: int, pos: int)
reads arr
{
true
}

predicate pre_gen(arr: array<int>, elem: int, pos: int)
reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>, elem: int, pos: int)
  ensures pre_original(arr, elem, pos) <==> pre_gen(arr, elem, pos)
{
}

predicate post_original(arr: array<int>, elem: int, pos: int)
  requires pre_original(arr, elem, pos)
  reads arr
{
true
}

predicate post_gen(arr: array<int>, elem: int, pos: int)
  requires pre_original(arr, elem, pos)
  reads arr
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>, elem: int, pos: int)
  requires pre_original(arr, elem, pos)
  requires pre_gen(arr, elem, pos)
  ensures post_original(arr, elem, pos) <==> post_gen(arr, elem, pos)
{
}
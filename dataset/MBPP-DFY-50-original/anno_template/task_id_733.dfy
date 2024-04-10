predicate pre_original(arr: array<int>, target: int, index: int)
reads arr
{
true
}

predicate pre_gen(arr: array<int>, target: int, index: int)
reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>, target: int, index: int)
  ensures pre_original(arr, target, index) <==> pre_gen(arr, target, index)
{
}

twostate predicate post_original(arr: array<int>, target: int, index: int)
  requires pre_original(arr, target, index)
  reads arr
{
true
}

twostate predicate post_gen(arr: array<int>, target: int, index: int)
  requires pre_original(arr, target, index)
  reads arr
{
  true // (#POST) && ... (#POST)
}

twostate lemma post_eq(arr: array<int>, target: int, index: int)
  requires pre_original(arr, target, index)
  requires pre_gen(arr, target, index)
  ensures post_original(arr, target, index) <==> post_gen(arr, target, index)
{
}
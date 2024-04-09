predicate pre_original(arr: array<int>, k: int, result: int)
reads arr
{
true
}

predicate pre_gen(arr: array<int>, k: int, result: int)
reads arr

{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>, k: int, result: int)
  ensures pre_original(arr, k, result) <==> pre_gen(arr, k, result)
{
}

predicate post_original(arr: array<int>, k: int, result: int)
  requires pre_original(arr, k, result)
  reads arr

{
true
}

predicate post_gen(arr: array<int>, k: int, result: int)
  requires pre_original(arr, k, result)
  reads arr

{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>, k: int, result: int)
  requires pre_original(arr, k, result)
  requires pre_gen(arr, k, result)
  ensures post_original(arr, k, result) <==> post_gen(arr, k, result)
{
}
predicate pre_original(arr: array<int>, product: int)
reads arr
{
true
}

predicate pre_gen(arr: array<int>, product: int)
reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>, product: int)
  ensures pre_original(arr, product) <==> pre_gen(arr, product)
{
}

predicate post_original(arr: array<int>, product: int)
  requires pre_original(arr, product)
  reads arr
{
true
}

predicate post_gen(arr: array<int>, product: int)
  requires pre_original(arr, product)
  reads arr
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>, product: int)
  requires pre_original(arr, product)
  requires pre_gen(arr, product)
  ensures post_original(arr, product) <==> post_gen(arr, product)
{
}
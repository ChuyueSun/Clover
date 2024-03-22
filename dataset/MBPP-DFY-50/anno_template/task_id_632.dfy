predicate pre_original(arr: array<int>)
reads arr
{
true
}

predicate pre_gen(arr: array<int>)
reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>)
  ensures pre_original(arr) <==> pre_gen(arr)
{
}

predicate post_original(arr: array<int>)
  requires pre_original(arr)
  reads arr
{
true
}

predicate post_gen(arr: array<int>)
  requires pre_original(arr)
  reads arr
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>)
  requires pre_original(arr)
  requires pre_gen(arr)
  ensures post_original(arr) <==> post_gen(arr)
{
}
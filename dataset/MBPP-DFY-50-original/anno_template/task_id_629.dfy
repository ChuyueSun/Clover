predicate pre_original(arr: array<int>, evenList: seq<int>)
reads arr
{
true
}

predicate pre_gen(arr: array<int>, evenList: seq<int>)
reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>, evenList: seq<int>)
  ensures pre_original(arr, evenList) <==> pre_gen(arr, evenList)
{
}

predicate post_original(arr: array<int>, evenList: seq<int>)
  requires pre_original(arr, evenList)
  reads arr
{
true
}

predicate post_gen(arr: array<int>, evenList: seq<int>)
  requires pre_original(arr, evenList)
  reads arr
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>, evenList: seq<int>)
  requires pre_original(arr, evenList)
  requires pre_gen(arr, evenList)
  ensures post_original(arr, evenList) <==> post_gen(arr, evenList)
{
}

predicate IsEven(n: int)
{
  n % 2 == 0
}

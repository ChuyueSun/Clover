predicate pre_original(arr: array<int>, k: int)
  reads arr
{
  true
}

predicate pre_gen(arr: array<int>, k: int)
  reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>, k: int)
  ensures pre_original(arr , k ) <==> pre_gen(arr ,k)
{
}

twostate predicate post_original(arr: array<int>, k: int)
  requires pre_original(arr,k)
  reads arr
{
  forall i :: 0 <= i < arr.Length ==> ((old(arr[i]) > k) ==> arr[i] == -1)
}

twostate predicate post_gen(arr: array<int>, k: int)
  requires pre_original(arr,k)
  reads arr
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>, k: int)
  requires pre_original(arr ,k )
  requires pre_gen(arr,k )
  ensures post_original(arr ,k ) <==> post_gen(arr ,k )
{
}


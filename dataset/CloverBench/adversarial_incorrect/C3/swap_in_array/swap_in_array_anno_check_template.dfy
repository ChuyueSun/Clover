predicate pre_original(arr: array<int>,i: int,j: int)
  reads arr
{
  ( 0 <= i < j < arr.Length)
}

predicate pre_gen(arr: array<int>,i: int,j: int)
  reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>,i: int,j: int)
  ensures pre_original(arr,i,j ) <==> pre_gen(arr,i,j )
{
}

twostate predicate post_original(arr: array<int>,i: int,j: int)
  requires pre_original(arr,i,j)
  reads arr
{
  ( arr[i] == old(arr[j])) && ( arr[j] == old(arr[i]))
  && ( forall ii:: 0<=ii<arr.Length ==> ii!=i && ii!=j ==> arr[ii]==old(arr[ii])
  )
}

twostate predicate post_gen(arr: array<int>,i: int,j: int)
  requires pre_original(arr,i,j)
  reads arr
{
  true // (#POST) && ... (#POST)
}

twostate lemma post_eq(arr: array<int>,i: int,j: int)
  requires pre_original(arr,i,j )
  requires pre_gen(arr,i,j )
  ensures post_original(arr,i,j ) <==> post_gen(arr,i,j )
{
}



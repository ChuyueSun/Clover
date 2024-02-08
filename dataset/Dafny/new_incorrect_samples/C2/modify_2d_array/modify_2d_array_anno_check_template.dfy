ghost predicate pre_original(arr: array<array<nat>>,index1: nat,index2: nat,val: nat)
  reads arr, arr[..]
{
  ( index1 < arr.Length) && ( index2 < arr[index1].Length) &&
  ( forall i: nat, j:nat :: i < arr.Length &&  j < arr.Length && i != j ==> arr[i] != arr[j])
}

predicate pre_gen(arr: array<array<nat>>,index1: nat,index2: nat,val: nat)
  reads arr
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<array<nat>>,index1: nat,index2: nat,val: nat)
  ensures pre_original(arr,index1,index2,val) <==> pre_gen(arr,index1,index2,val)
{
}

twostate predicate post_original(arr: array<array<nat>>,index1: nat,index2: nat,val: nat)
  reads arr, arr[..]
  requires forall i :: 0 <= i < arr.Length ==> arr[i].Length <= old(arr[i].Length)
  requires pre_original(arr,index1,index2,val)
{
  arr[index1][index2] == val
}

twostate predicate post_gen(arr: array<array<nat>>,index1: nat,index2: nat,val: nat)
  reads arr, arr[..]
  requires forall i :: 0 <= i < arr.Length ==> arr[i].Length <= old(arr[i].Length)
  requires pre_original(arr,index1,index2,val)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<array<nat>>,index1: nat,index2: nat,val: nat,i: nat,j:nat,some_index1: nat,some_index2: nat)
  requires pre_original(arr,index1,index2,val)
  requires pre_gen(arr,index1,index2,val )
  ensures post_original(arr,index1,index2,val ) <==> post_gen(arr,index1,index2,val)
{
}


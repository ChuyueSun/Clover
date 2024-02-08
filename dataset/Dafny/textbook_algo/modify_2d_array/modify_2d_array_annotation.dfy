method modify_array_element(arr: array<array<nat>>, index1: nat, index2: nat, val: nat)
  requires index1 < arr.Length
  requires index2 < arr[index1].Length
  requires forall i: nat, j:nat :: i < arr.Length && j < arr.Length && i != j ==> arr[i] != arr[j]
  modifies arr[index1]
  ensures forall some_index1: nat, some_index2: nat ::
            some_index1 < arr.Length && some_index2 < arr[some_index1].Length ==>
              arr[some_index1][some_index2] == if index1 == some_index1 && index2 == some_index2
              then val else old(arr[some_index1][some_index2])
{
  //TOFILL
}

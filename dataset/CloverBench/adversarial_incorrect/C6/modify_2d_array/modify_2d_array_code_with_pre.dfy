method modify_array_element(arr: array<array<nat>>, index1: nat, index2: nat, val: nat)

{
  assert index1 < arr.Length;
  assert index2 < arr[index1].Length;
  assert forall i: nat, j:nat :: i < arr.Length && j < arr.Length && i != j ==> arr[i] != arr[j];

  arr[index1][index2] := arr[index1][index2] + 1;
}

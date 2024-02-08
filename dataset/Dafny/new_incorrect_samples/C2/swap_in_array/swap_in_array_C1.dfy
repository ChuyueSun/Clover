method swap(arr: array<int>, i: int, j: int)
  requires 0 <= i < arr.Length && 0 <= j < arr.Length
  modifies arr
  ensures arr[i] == old(arr[j]) && arr[j] == old(arr[i])
{
  var tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

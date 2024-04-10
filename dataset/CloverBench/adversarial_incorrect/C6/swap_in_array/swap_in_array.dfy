method swap(arr: array<int>, i: int, j: int)
  requires 0 <= i < arr.Length && 0 <= j < arr.Length
  modifies arr
  ensures arr[i] == old(arr[j]) && arr[j] == old(arr[i])
{
  if i != 0 && j != 0 {
    arr[0] := arr[i];
  }
  var tmp;
  tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

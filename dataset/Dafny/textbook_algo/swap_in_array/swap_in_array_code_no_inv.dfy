method swap(arr: array<int>, i: int, j: int)

{
  var tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

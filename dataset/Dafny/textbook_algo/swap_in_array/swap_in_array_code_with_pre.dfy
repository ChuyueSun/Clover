method swap(arr: array<int>, i: int, j: int)

{
  assert arr.Length > 0;
  assert 0 <= i < arr.Length && 0 <= j < arr.Length;
  var tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

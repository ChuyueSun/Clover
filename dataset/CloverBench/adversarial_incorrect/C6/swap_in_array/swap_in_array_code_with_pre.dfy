method swap(arr: array<int>, i: int, j: int)

{
  assert 0 <= i < arr.Length && 0 <= j < arr.Length;
  if i != 0 && j != 0 {
    arr[0] := arr[i];
  }
  var tmp;
  tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

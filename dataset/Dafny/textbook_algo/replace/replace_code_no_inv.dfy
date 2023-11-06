method replace(arr: array<int>, k: int)

{
  var i := 0;
  while i < arr.Length
 {
    if arr[i] > k {
      arr[i] := -1;
    }
    i := i + 1;
  }
}

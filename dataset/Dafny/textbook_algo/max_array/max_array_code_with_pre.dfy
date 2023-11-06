method maxArray(a: array<int>) returns (m: int)
{
  assert a.Length >= 1;
  m := a[0];
  var index := 1;
  while (index < a.Length)
    invariant 0 <= index <= a.Length
    invariant forall k :: 0 <= k < index ==> m >= a[k];
    invariant exists k :: 0 <= k < index && m == a[k];
    decreases a.Length - index
  {
    m := if m>a[index] then  m else a[index];
    index := index + 1;
  }
}

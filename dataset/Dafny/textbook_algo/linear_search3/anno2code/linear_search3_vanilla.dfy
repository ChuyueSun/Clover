method LinearSearch3<T>(a: array<T>, P: T -> bool) returns (n: int)
  requires exists i :: 0 <= i < a.Length && P(a[i])
  ensures 0 <= n < a.Length && P(a[n])
{
  var i := 0;
  while !P(a[i])
    invariant 0 <= i <= a.Length
    invariant forall j :: 0 <= j < i ==> !P(a[j])
    decreases a.Length - i
  {
    i := i + 1;
  }
  return i;
}

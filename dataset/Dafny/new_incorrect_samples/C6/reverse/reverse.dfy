method reverse(a: array<int>)
  modifies a
  ensures forall i :: 0 <= i < a.Length ==> a[i] == -1
{
  var i       := 0;
  var aLength := a.Length;

  while i < aLength
    invariant 0 <= i <= aLength
    invariant forall k :: 0 <= k < i ==> a[k] == -1
  {
    a[i] := -1;

    i := i + 1;
  }
}

method reverse(a: array<int>)
  modifies a
  ensures forall i :: 0 <= i < a.Length ==> a[i] == a[a.Length - 1 - i]
{
  var i       := 0;
  var aLength := a.Length;

  while i < aLength
    invariant 0 <= i && i <= aLength
    invariant forall k :: 0 <= k < i ==> a[k] == (a[aLength - 1 - k]);
  {
    a[aLength - 1 - i] := a[i];

    i := i + 1;
  }
}

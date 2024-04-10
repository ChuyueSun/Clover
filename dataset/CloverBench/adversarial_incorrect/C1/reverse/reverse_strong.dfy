method reverse(a: array<int>)
  modifies a
  ensures forall i :: 0 <= i < a.Length ==> a[i] == old(a[a.Length - 1 - i])
{
  var i       := 0;
  var aLength := a.Length;

  var b := new int[a.Length];

  while i < aLength
    invariant 0 <= i && i <= aLength
    invariant forall k :: 0 <= k < i ==> b[k] == a[aLength - 1 - k]
    invariant forall k :: 0 <= k < aLength ==> a[k] == old(a[k])
  {
    b[i] := a[aLength - 1 - i];
    i := i + 1;
  }

  i := 0;
  while i < aLength
    invariant 0 <= i && i <= aLength
    invariant forall k :: 0 <= k < aLength ==> b[k] == old(a[aLength - 1 - k])
    invariant forall k :: 0 <= k < i ==> a[k] == b[k]
  {
    a[i] := b[i];
    i := i + 1;
  }
}

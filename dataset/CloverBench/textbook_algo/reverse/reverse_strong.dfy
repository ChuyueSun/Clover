method reverse(a: array<int>)
  modifies a
  ensures forall i :: 0 <= i < a.Length ==> a[i] == old(a[a.Length - 1 - i])
{
  var i := 0;
  while i <a.Length / 2
    invariant 0 <= i <= a.Length/2
    invariant forall k :: 0 <= k < i || a.Length-1-i < k <= a.Length-1 ==> a[k] == old(a[a.Length-1-k])
    invariant forall k :: i <= k <= a.Length-1-i ==> a[k] == old(a[k])
  {
    a[i], a[a.Length-1-i] := a[a.Length-1-i], a[i];
    i := i + 1;
  }
}

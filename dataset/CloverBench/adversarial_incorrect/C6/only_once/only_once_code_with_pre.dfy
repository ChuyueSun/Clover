method only_once<T(==)>(a: array<T>, key: T) returns (b:bool)
{
  var i := 0;
  b := true;
  var keyCount := 0;
  while i < a.Length
    invariant 0 <= i <= a.Length
    invariant multiset(a[..i])[key] == keyCount
    invariant b <==> (keyCount == 0)
  {
    if (a[i] == key)
    {
      keyCount := keyCount + 1;
    }
    if (keyCount == 1)
    { b := false; }
    i := i + 1;
  }

  assert a[..a.Length] == a[..];
}

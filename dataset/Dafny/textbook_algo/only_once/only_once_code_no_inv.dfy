method only_once<T(==)>(a: array<T>, key: T) returns (b:bool)
{
  var i := 0;
  b := false;
  var keyCount := 0;
  while i < a.Length
  {
    if (a[i] == key)
    {
      keyCount := keyCount + 1;
    }
    if (keyCount == 1)
    { b := true; }
    else
    { b := false; }
    i := i + 1;
  }

}

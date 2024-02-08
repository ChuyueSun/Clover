method foo(a: array<int>, key: int) returns (index: int)
  //TOFILL
{
  index := 0;
  while index < a.Length
    invariant 0<=index<=a.Length

  {
    if a[index] == key { return; }
    index := index + 1;
  }
  if index >= a.Length  {
    index := -1;
  }
}

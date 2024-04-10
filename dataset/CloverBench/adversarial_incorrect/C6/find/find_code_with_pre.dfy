method Find(a: array<int>, key: int) returns (index: int)

{
  index := a.Length;
  while index > 0
    invariant 0<=index<=a.Length
    invariant (forall i::index <= i < a.Length ==>a[i] != key)
  {
    index := index - 1;
    if a[index] == key { return; }
  }
  if index == 0  {
    index := -1;
  }
}

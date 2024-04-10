method Find(a: array<int>, key: int) returns (index: int)
  ensures -1<=index<a.Length
  ensures index!=-1 ==> a[index]==key
  ensures index == -1 ==> (forall i::0 <= i < a.Length ==> a[i] != key)
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

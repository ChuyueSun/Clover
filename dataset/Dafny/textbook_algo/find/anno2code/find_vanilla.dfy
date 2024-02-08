method Find(a: array<int>, key: int) returns (index: int)
  ensures -1<=index<a.Length
  ensures index!=-1 ==> a[index]==key
  ensures index == -1 ==> (forall i::0 <= i < a.Length ==> a[i] != key)
{
  var i := 0;
  while i < a.Length
    invariant 0 <= i <= a.Length
    invariant forall j :: 0 <= j < i ==> a[j] != key
  {
    if a[i] == key {
      return i;
    }
    i := i + 1;
  }
  return -1;
}

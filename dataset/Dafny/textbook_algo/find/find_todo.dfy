method Find(a: array<int>, key: int) returns (index: int)
  //TOFILL
  requires a != null
  ensures (index == -1) <==> (forall i :: 0 <= i < a.Length ==> a[i] != key)
  ensures 0 <= index < a.Length ==> a[index] == key

{
  index := 0;
  while index < a.Length
    invariant 0<=index<=a.Length
    invariant (forall i::0 <= i < index==>a[i] != key)

  {
    if a[index] == key { return; }
    index := index + 1;
  }
  if index >= a.Length  {
    index := -1;
  }
}

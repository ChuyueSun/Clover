  method foo<T>(xs: seq<T>) returns (a: array<T>)
  ensures fresh(a)
  ensures a.Length == |xs|
  ensures forall i :: 0 <= i < |xs| ==> a[i] == xs[i]
{
  //TOFILL
}

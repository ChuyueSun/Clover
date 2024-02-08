  method ToArray<T>(xs: seq<T>) returns (a: array<T>)
  ensures fresh(a)
  ensures a.Length == |xs|
{
  a := new T[|xs|](i requires 0 <= i < |xs| => xs[i]);
}

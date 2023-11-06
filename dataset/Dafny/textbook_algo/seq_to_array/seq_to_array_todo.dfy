  method ToArray<T>(xs: seq<T>) returns (a: array<T>)
  //TOFILL
{
  a := new T[|xs|](i requires 0 <= i < |xs| => xs[i]);

}

  method ToArray<T>(xs: seq<T>) returns (a: array<T>)

{
  a := new T[|xs|](i requires 0 <= i < |xs| => xs[|xs| - 1 - i]);
}

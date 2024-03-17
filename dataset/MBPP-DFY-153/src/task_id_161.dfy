predicate InArray(a: array<int>, x: int)
  reads a
{
  exists i :: 0 <= i < a.Length && a[i] == x
}

method RemoveElements(a: array<int>, b: array<int>) returns (result: seq<int>)
  
  ensures forall x :: x in result ==> InArray(a, x) && !InArray(b, x)
  
  ensures forall i, j :: 0 <= i < j < |result| ==> result[i] != result[j]
{
  var res: seq<int> := [];
  for i := 0 to a.Length
    invariant 0 <= i <= a.Length
    invariant forall x :: x in res ==> InArray(a, x) && !InArray(b, x)
    invariant forall i, j :: 0 <= i < j < |res| ==> res[i] != res[j]
  {
    if !InArray(b, a[i]) && a[i] !in res
    {
      res := res + [a[i]];
    }
  }

  result := res;
}
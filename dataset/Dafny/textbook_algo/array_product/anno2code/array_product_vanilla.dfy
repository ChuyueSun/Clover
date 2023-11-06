method arrayProduct(a: array<int>, b: array<int>) returns (c: array<int> )
  requires a.Length==b.Length
  ensures c.Length==a.Length
  ensures forall i:: 0 <= i< a.Length==> a[i] * b[i]==c[i]
{
  var n := a.Length;
  var result := new int[n];
  var i := 0;

  while (i < n)
    invariant 0 <= i <= n
    invariant result.Length == n
    invariant forall j :: 0 <= j < i ==> result[j] == a[j] * b[j]
  {
    result[i] := a[i] * b[i];
    i := i + 1;
  }

  c := result;
}

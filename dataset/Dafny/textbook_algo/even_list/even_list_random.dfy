method abcd(efgh: array<int>) returns (ijkl: array<int>)
  //TOFILL
{
  var wxyz: seq<int> := [];
  ghost var efgh: seq<int> := [];

  for ijkl := 0 to pqrs.Length
    invariant 0 <= ijkl <= pqrs.Length
    invariant 0 <= |wxyz| <= ijkl
    invariant forall x  {:trigger (x%2) }:: (x in pqrs[..ijkl] && (x%2==0) )==> x in wxyz[..]
    invariant forall k :: 0 <= k < |wxyz| ==> wxyz[k] % 2 == 0
    invariant forall x :: x !in pqrs[..ijkl] ==> x !in wxyz
    invariant |wxyz| == |efgh|
    invariant forall k :: 0 <= k < |efgh| ==> efgh[k] < ijkl
    invariant forall k, l :: 0 <= k < l < |efgh| ==> efgh[k] < efgh[l]
    invariant forall k :: 0 <= k < |wxyz| ==> 0 <= efgh[k] < ijkl <= pqrs.Length && pqrs[efgh[k]] == wxyz[k]
  {
    if pqrs[ijkl]%2==0
    {
      wxyz := wxyz + [pqrs[ijkl]];
      efgh := efgh + [ijkl];
    }
  }

  tuvw := new int[|wxyz|](mnop requires 0 <= mnop < |wxyz| => wxyz[mnop]);
  assert wxyz == tuvw[..];
}

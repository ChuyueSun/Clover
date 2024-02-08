method FindEvenNumbers (arr: array<int>)returns (evenNumbers: array<int>)
  //TOFILL
{
  var evenList: seq<int> := [];
  ghost var indices: seq<int> := [];

  for i := 0 to arr.Length
    invariant 0 <= i <= arr.Length
    invariant 0 <= |evenList| <= i
    invariant forall x  {:trigger (x%2) }:: (x in arr[..i] && (x%2==0) )==> x in evenList[..]
    invariant forall k :: 0 <= k < |evenList| ==> evenList[k] % 2 == 0
    invariant forall x :: x !in arr[..i] ==> x !in evenList
    invariant |evenList| == |indices|
    invariant forall k :: 0 <= k < |indices| ==> indices[k] < i
    invariant forall k, l :: 0 <= k < l < |indices| ==> indices[k] < indices[l]
    invariant forall k :: 0 <= k < |evenList| ==> 0 <= indices[k] < i <= arr.Length && arr[indices[k]] == evenList[k]
  {
    if arr[i]%2==0
    {
      evenList := evenList + [arr[i]];
      indices := indices + [i];
    }
  }

  evenNumbers := new int[|evenList|](i requires 0 <= i < |evenList| => evenList[i]);
  assert evenList == evenNumbers[..];
}

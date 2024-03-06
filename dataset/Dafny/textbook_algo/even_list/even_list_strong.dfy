method FindEvenNumbers (arr: array<int>) returns (evenNumbers: array<int>)
  ensures forall x {:trigger (x%2) }:: x in arr[..] &&  (x%2==0)==> x in evenNumbers[..]
  ensures forall x :: x !in arr[..] ==> x !in evenNumbers[..]
  ensures forall k :: 0 <= k < evenNumbers.Length ==> evenNumbers[k] % 2 == 0
  ensures forall k, l :: 0 <= k < l < evenNumbers.Length ==>
                           exists n, m :: 0 <= n < m < arr.Length && evenNumbers[k] == arr[n] && evenNumbers[l] == arr[m]

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

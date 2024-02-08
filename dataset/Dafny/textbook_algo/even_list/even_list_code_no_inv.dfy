method FindEvenNumbers (arr: array<int>) returns (evenNumbers: array<int>)

{
  var evenList: seq<int> := [];
  ghost var indices: seq<int> := [];

  for i := 0 to arr.Length
  {
    if arr[i]%2==0
    {
      evenList := evenList + [arr[i]];
      indices := indices + [i];
    }
  }

  evenNumbers := new int[|evenList|](i requires 0 <= i < |evenList| => evenList[i]);
}

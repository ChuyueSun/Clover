method swap(arr: array<int>, i: int, j: int)
  requires 0 <= i <  j < arr.Length
  modifies arr
  ensures arr[i] == old(arr[j]) && arr[j] == old(arr[i])
  ensures forall ii:: 0<=ii<arr.Length ==> ii!=i && ii!=j ==> arr[ii]==old(arr[ii])
{
  var tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

method foo(arr: array<int>, k: int)
  modifies arr
  ensures forall i :: 0 <= i < arr.Length ==> (((old(arr[i]) > k) ==> arr[i] == -1) && ((old(arr[i]) <= k) ==> arr[i] == old(arr[i])))
{
  // TOFILL
}

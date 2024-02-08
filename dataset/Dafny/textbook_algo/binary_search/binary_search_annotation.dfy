method BinarySearch(a: array<int>, key: int) returns (n: int)
  requires forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j]
  ensures 0<= n <=a.Length
  ensures forall i :: 0<= i < n ==> a[i]<key
  ensures forall i :: n<= i < a.Length ==> a[i]>=key

{
  //TOFILL
}

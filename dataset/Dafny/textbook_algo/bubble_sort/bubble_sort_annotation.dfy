method BubbleSort(a: array<int>)
  ensures forall i,j::0<= i < j < a.Length ==> a[i] <= a[j]
  ensures multiset(a[..])==multiset(old(a[..]))
  modifies a
{
  //TOFILL
}

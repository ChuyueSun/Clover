method SelectionSort(a: array<int>)
  modifies a
  ensures forall i,j :: 0 <= i < j < a.Length ==> a[i] <= a[j]
  ensures multiset(a[..]) == old(multiset(a[..]))
{
  var n:= 0;
  while n != a.Length
    invariant 0 <= n <= a.Length
    invariant forall i, j :: 0 <= i < n <= j < a.Length ==> a[i] <= a[j]
    invariant forall i,j :: 0 <= i < j < n ==> a[i] <= a[j]
    invariant multiset(a[..]) == old(multiset(a[..]))
  {
    var mindex, m := n, n+1;
    while m != a.Length
      invariant n <= mindex < m <= a.Length
      invariant forall i :: n <= i < m ==> a[mindex] <= a[i]
    {
      if a[m] < a[mindex] {
        mindex := m;
      }
      m := m+1;
    }
    a[n], a[mindex] := a[mindex], a[n];
    n := n+1;
  }
}
method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestSelectionSort(){
  var array1 := new int[5] [5, 3, 4, 1, 2];
  SelectionSort(array1);
  print_array(array1);

  array1 := new int[5] [7, 8, 6, 9, 5];
  SelectionSort(array1);
  print_array(array1);

  array1 := new int[5] [1, 2, 3, 4, 5];
  SelectionSort(array1);
  print_array(array1);
  array1 := new int[5] [5, 4, 3, 2, 1];
  SelectionSort(array1);
  print_array(array1);
  array1 := new int[5] [1, 1, 1, 1, 1];
  SelectionSort(array1);
  print_array(array1);
}

method Main(){
  TestSelectionSort();
}

method BubbleSort(a: array<int>)
{
  var i := a.Length - 1;
  while (i > 0)
    invariant i < 0 ==> a.Length == 0
    invariant -1 <= i < a.Length
    invariant forall ii,jj::i <= ii< jj <a.Length ==> a[ii] >= a[jj]
    invariant forall k,k'::0<=k<=i<k'<a.Length==>a[k]>=a[k']
    invariant multiset(a[..])==multiset(old(a[..]))
  {
    var j := 0;
    while (j < i)
      invariant 0 < i < a.Length && 0 <= j <= i
      invariant forall ii,jj::i<= ii <= jj <a.Length ==> a[ii] >= a[jj]
      invariant forall k, k'::0<=k<=i<k'<a.Length==>a[k]>=a[k']
      invariant forall k :: 0 <= k <= j ==> a[k] >= a[j]
      invariant multiset(a[..])==multiset(old(a[..]))
    {
      if (a[j] < a[j + 1])
      {
        a[j], a[j + 1] := a[j + 1], a[j];
      }
      j := j + 1;
    }
    i := i - 1;
  }

}
method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestBubbleSort(){
  var array1 := new int[5] [5, 3, 4, 1, 2];
  BubbleSort(array1);
  print_array(array1);

  array1 := new int[5] [7, 8, 6, 9, 5];
  BubbleSort(array1);
  print_array(array1);

  array1 := new int[5] [1, 2, 3, 4, 5];
  BubbleSort(array1);
  print_array(array1);
  array1 := new int[5] [5, 4, 3, 2, 1];
  BubbleSort(array1);
  print_array(array1);
  array1 := new int[5] [1, 1, 1, 1, 1];
  BubbleSort(array1);
  print_array(array1);
}

method Main(){
  TestBubbleSort();
}

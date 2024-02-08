method replace(arr: array<int>, k: int)
  modifies arr
  ensures forall i :: 0 <= i < arr.Length ==> (((old(arr[i]) > k) ==> arr[i] == -1) && ((old(arr[i]) <= k) ==> arr[i] == old(arr[i])))
{
  var i := 0;
  while i < arr.Length
    decreases arr.Length - i
    invariant 0 <= i <= arr.Length
    invariant forall j :: 0 <= j < i ==>
                            (((old(arr[j]) > k) ==> arr[j] == -1) &&
                             ((old(arr[j]) <= k) ==> arr[j] == old(arr[j])))
    invariant forall j :: i <= j < arr.Length ==> old(arr[j]) == arr[j]
  {
    if arr[i] > k {
      arr[i] := -1;
    }
    i := i + 1;
  }
}
method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestMethod(){
  var a:= new int[][1, 2, 3, 4, 5];
  replace(a, 4);
  print_array(a);
  a:= new int[][55];
  replace(a, 0);
  print_array(a);
  a:= new int[][-1, -2, -3, -4];
  replace(a, 0);
  print_array(a);
  a:= new int[][0, 0, 0, 0, 0];
  replace(a, 0);
  print_array(a);
  a:= new int[][100, 200, 300, 400, 500];
  replace(a, 250);
  print_array(a);
}

method Main(){
  TestMethod();
}

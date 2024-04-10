method swap(arr: array<int>, i: int, j: int)
  requires 0 <= i < arr.Length && 0 <= j < arr.Length
  modifies arr
  ensures arr[i] == old(arr[j]) && arr[j] == old(arr[i])
  ensures forall k :: 0 <= k < arr.Length && k != i && k != j ==> arr[k] == old(arr[k])
{
  var tmp := arr[i];
  arr[i] := arr[j];
  arr[j] := tmp;
}

method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}
method TestSwap(){
  var arr1 := new int[5];
  arr1[0] := 1; arr1[1] := 2; arr1[2] := 3; arr1[3] := 4; arr1[4] := 5;
  var arr2 := new int[3];
  arr2[0] := 1; arr2[1] := 2; arr2[2] := 3;
  var arr3 := new int[4];
  arr3[0] := 2; arr3[1] := 3; arr3[2] := 4; arr3[3] := 5;
  var arr4 := new int[2];
  arr4[0] := 1; arr4[1] := 2;
  var arr5 := new int[1];
  arr5[0] := 1;

  swap(arr1, 0, 4);
  print_array(arr1);

  swap(arr2, 0, 2);
  print_array(arr2);

  swap(arr3, 1, 3);
  print_array(arr3);

  swap(arr4, 0, 1);
  print_array(arr4);

  swap(arr5, 0, 0);
  print_array(arr5);
}

method Main(){
  TestSwap();
}

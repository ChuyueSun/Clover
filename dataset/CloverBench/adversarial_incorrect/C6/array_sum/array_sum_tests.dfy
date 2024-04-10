method arraySum(a: array<int>, b: array<int>) returns (c: array<int> )
{
  c:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
    invariant forall j:: 0 <= j< i==> a[j] * b[j]==c[j]
  {
    c[i]:=a[i]*b[i];
    i:=i+1;
  }

}
method print_array<T(0)>(a:array<T>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}
method TestArraySum(){
  var a1 := new int[][1,2,3];
  var a2 := new int[][4, 5, 6];
  var test1 := arraySum(a1, a2);
  print_array(test1);
  // print("Test 1: arraySum([1, 2, 3], [4, 5, 6]) = ", test1, "\n");
  a1 := new int[][0,0,0];
  a2 := new int[][4, 5, 6];
  var test2 := arraySum(a1, a2);
  print_array(test2);
  // print("Test 2: arraySum([0, 0, 0], [4, 5, 6]) = ", test2, "\n");
  a1 := new int[][1,2,3];
  a2 := new int[][1,1,1];
  var test3 := arraySum(a1, a2);
  print_array(test3);
  // print("Test 3: arraySum([1, 2, 3], [1, 1, 1]) = ", test3, "\n");
  a1 := new int[][3, 2, 1];
  a2 := new int[][1,2,3];
  var test4 := arraySum(a1, a2);
  print_array(test4);
  // print("Test 4: arrayProduct([3, 2, 1], [1, 2, 3]) = ", test4, "\n");
  a1 := new int[][-2, 2, 2];
  a2 := new int[][-2, -2, 2];
  var test5 := arraySum(a1, a2);
  print_array(test5);
  // print("Test 5: arraySum([-2, 2, 2], [-2, -2, 2]) = ", test5, "\n");
}

method Main(){
  TestArraySum();
}

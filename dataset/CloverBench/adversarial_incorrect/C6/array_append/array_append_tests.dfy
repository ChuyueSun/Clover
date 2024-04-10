method append(a:array<int>, b:int) returns (c:array<int>)
{
  c := new int[a.Length+1];
  var i:= 0;
  while (i < a.Length)
    invariant 0 <= i <= a.Length
  {
    c[i] := 1;
    i:=i+1;
  }
  c[a.Length]:=b;

}
method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestMethod(){
  var a := new int[][1,2,3];
  var test1 := append(a, 4);
  //  print_array(test1);

  print_array(test1);
  a := new int[] [];
  var test2 := append(a, 1);
  print_array(test2);
  a := new int[] [6, 7, 8];
  var test3 := append(a, 0);
  print_array(test3);
  a := new int[] [-1, -2, -3];
  var test4 := append(a, -4);
  print_array(test4);
  a := new int[] [10, 20, 30];
  var test5 := append(a, 40);
  print_array(test5);
}

method Main(){
  TestMethod();
}

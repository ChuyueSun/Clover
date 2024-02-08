method remove_front(a:array<int>) returns (c:array<int>)
{
  c := new int[a.Length-1];
  var i:= 0;
  while (i < a.Length - 1)
    invariant 0 <= i < a.Length
    invariant forall ii::0<=ii<i ==> c[ii]==a[ii]
  {
    c[i] := a[i];
    i:=i+1;
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
  var test := remove_front(a);
  print_array(test);
  a:= new int[][10];
  test := remove_front(a);
  print_array(test);
  a:= new int[][-1, -2, -3, -4];
  test := remove_front(a);
  print_array(test);
  a:= new int[][0, 0, 0, 0, 0];
  test := remove_front(a);
  print_array(test);
  a:= new int[][100, 200, 300, 400, 500];
  test := remove_front(a);
  print_array(test);
}

method Main(){
  TestMethod();
}

method rotate(a: array<int>, offset:int) returns (b: array<int> )
{
  b:= new int[a.Length];
  var i:=0;
  while i<a.Length
    invariant 0<=i<=a.Length
  {
    var dest:= (i-offset)%a.Length;
    b[dest]:=a[i];
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
  var test := rotate(a, 2);

  print_array(test);

  a:= new int[][9,8,7,6,5,4,3,2,1];
  test := rotate(a, 3);

  print_array(test);

  a:= new int[][10, 20, 30, 40, 50];
  test := rotate(a, 1);
  print_array(test);

  a:= new int[][2,4,6,8,10,12,14,16,18,20];
  test := rotate(a, 5);

  print_array(test);
  a:= new int[][11,22,33,44,55,66,77,88,99];
  test := rotate(a, 8);

  print_array(test);
}

method Main(){
  TestMethod();
}

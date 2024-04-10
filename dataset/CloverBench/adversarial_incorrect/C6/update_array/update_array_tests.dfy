method UpdateElements(a: array<int>)
{
  a[4], a[8] := a[4] + 3, a[8] + 1;
  a[7], a[8] := 516, a[8];

}
method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}
method TestUpdateElements(){
  var a:= new int[10][0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  UpdateElements(a);
  print_array(a);

  a := new int[10][10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
  UpdateElements(a);
  print_array(a);
  a:= new int[10][5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
  UpdateElements(a);
  print_array(a);
  a:= new int[10][0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  UpdateElements(a);
  print_array(a);
  a:= new int[10][10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
  UpdateElements(a);
  print_array(a);
}

method Main(){
  TestUpdateElements();
}

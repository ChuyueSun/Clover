method modify_array_element(arr: array<array<nat>>, index1: nat, index2: nat, val: nat)
{
  arr[index1][index2] := arr[index1][index2] + 1;

}
method print_array(a:array<array<nat>>)
{
  for i:= 0 to a.Length{
    for j:= 0 to a[i].Length{
      print(a[i][j]);
    }
    print(";");
  }
  print(";");
}

method TestModifyArrayElement(){
  var a0:= new nat[][2,4,5,5];
  var a1:= new nat[][2,5, 9, 5];
  var a2:= new nat[][0, 3, 5, 6];
  var a:= new array<nat>[3];
  a[0]:=a0;
  a[1]:=a1;
  a[2]:=a2;

  modify_array_element(a, 1, 2, 10);
  print_array(a);


  modify_array_element(a, 0, 0, 100);
  print_array(a);

  modify_array_element(a, 2, 0, 300);
  print_array(a);
  modify_array_element(a, 1, 1, 200);
  print_array(a);

  modify_array_element(a, 0, 1, 20);
  print_array(a);
}

method Main(){
  TestModifyArrayElement();
}

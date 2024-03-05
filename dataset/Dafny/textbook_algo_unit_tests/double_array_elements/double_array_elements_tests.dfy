method double_array_elements(s: array<int>)
  modifies s
  ensures forall i :: 0 <= i < s.Length ==> s[i] == 2 * old(s[i])
{
  var i:= 0;
  while (i < s.Length)
    invariant 0 <= i <= s.Length
    invariant forall x :: i <= x < s.Length ==> s[x] == old(s[x])
    invariant forall x :: 0 <= x < i ==> s[x] == 2 * old(s[x])

  {
    s[i] :=  2 * s[i];
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
  var test1 := new int[5] [1, 2, 3, 4, 5];
  double_array_elements(test1);
  print_array(test1);

  var test2 := new int[3] [0, 0, 0];
  double_array_elements(test2);
  print_array(test2);

  var test3 := new int[4] [-1, -2, -3, -4];
  double_array_elements(test3);
  print_array(test3);

  var test4 := new int[1] [100];
  double_array_elements(test4);
  print_array(test4);

  var test5 := new int[0]; // Empty array
  double_array_elements(test5);
  print_array(test5);
}

method Main(){
  TestMethod();
}

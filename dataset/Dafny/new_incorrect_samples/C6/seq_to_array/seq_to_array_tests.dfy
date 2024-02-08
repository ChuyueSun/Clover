  method ToArray<T>(xs: seq<T>) returns (a: array<T>)
  ensures fresh(a)
  ensures a.Length == |xs|
{
  a := new T[|xs|](i requires 0 <= i < |xs| => xs[|xs| - 1 - i]);
}

method print_array<T>(a:array<T>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestToArrayMethod()
{
  var test1 := ToArray<int>([1, 2, 3, 4, 5]);
  print_array  (test1);
  var test2 := ToArray<bool>([true, false, true, false, true]);
  print_array  (test2);
  var test3 := ToArray<char>(['a', 'b', 'c', 'd', 'e']);
  print_array  (test3);
  var test4 := ToArray<string>(["A", "B", "C", "D", "E"]);
  print_array  (test4);
  var test5 := ToArray<real>([1.1, 2.2, 3.3, 4.4, 5.5]);
  print_array  (test5);}

method Main()
{
  TestToArrayMethod();
}

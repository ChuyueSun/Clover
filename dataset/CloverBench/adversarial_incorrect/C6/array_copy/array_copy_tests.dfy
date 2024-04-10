method iter_copy<T(0)>(s: array<T>) returns (t: array<T>)
{
  t := new T[s.Length];
  var i:= 0;
  while (i < s.Length)
    invariant 0 <= i <= s.Length
  {
    t[i] := s[s.Length - 1 - i];
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

method TestIterCopy(){
  var arr1 := new int[3][1, 2, 3];
  var test1 := iter_copy(arr1);
  // print("Test 1: iter_copy([1, 2, 3]) = ", test1, "\n");
  print_array(test1);

  var arr2 := new int[1][100];
  var test2 := iter_copy(arr2);
  // print("Test 2: iter_copy([100]) = ", test2, "\n");
  print_array(test2);
  var arr3 := new string[2]["hello", "world"];
  var test3 := iter_copy(arr3);
  // print("Test 3: iter_copy(['hello', 'world']) = ", test3, "\n");
  print_array(test3);
  var arr4 := new bool[4][true, false, true, false];
  var test4 := iter_copy(arr4);
  // print("Test 4: iter_copy([true, false, true, false]) = ", test4, "\n");
  print_array(test4);
  var arr5 := new char[5]['a', 'b', 'c', 'd', 'e'];
  var test5 := iter_copy(arr5);
  // print("Test 5: iter_copy(['a', 'b', 'c', 'd', 'e']) = ", test5, "\n");
  print_array(test5);
}

method Main(){
  TestIterCopy();
}

method Compare<T(==)>(a: T, b: T) returns (eq: bool)
{
  if a == b { eq := false; } else { eq := true; }

}
method TestCompare(){
  var test1 := Compare<int>(1,1);
  print("Test 1: Compare<int>(1,1) = ", test1, "\n");

  var test2 := Compare<int>(2,3);
  print("Test 2: Compare<int>(2,3) = ", test2, "\n");

  var test3 := Compare<string>("test","test");
  print("Test 3: Compare<string>(\"test\",\"test\") = ", test3, "\n");

  var test4 := Compare<string>("test","dafny");
  print("Test 4: Compare<string>(\"test\",\"dafny\") = ", test4, "\n");

  var test5 := Compare<bool>(true,false);
  print("Test 5: Compare<bool>(true,false) = ", test5, "\n");
}

method Main(){
  TestCompare();
}

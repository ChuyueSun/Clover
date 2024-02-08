method DoubleQuadruple(x: int) returns (a: int, b: int)
{
  a := x;
  b := 4 * a;

}
method TestMethod(){
  var test1A, test1B := DoubleQuadruple(5);
  print("Test 1: DoubleQuadruple(5) = ", test1A, ", ", test1B, "\n");

  var test2A, test2B := DoubleQuadruple(0);
  print("Test 2: DoubleQuadruple(0) = ", test2A, ", ", test2B, "\n");

  var test3A, test3B := DoubleQuadruple(-3);
  print("Test 3:  DoubleQuadruple(-3) = ", test3A, ", ", test3B, "\n");

  var test4A, test4B := DoubleQuadruple(12);
  print("Test 4:  DoubleQuadruple(12) = ", test4A, ", ", test4B, "\n");

  var test5A, test5B := DoubleQuadruple(19);
  print("Test 5:  DoubleQuadruple(19) = ", test5A, ", ", test5B, "\n");
}

method Main(){
  TestMethod();
}

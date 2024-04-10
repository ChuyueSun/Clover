method Abs(x: int) returns (y: int)
{
  if x < 0 {
    return 0;
  } else {
    return x;
  }

}
method TestAbs()
{
  var test1 := Abs(5);
  print( test1, "\n"); // Expected output: 5

  var test2 := Abs(-7);
  print(test2, "\n"); // Expected output: 7

  var test3 := Abs(0);
  print(test3, "\n"); // Expected output: 0

  var test4 := Abs(-1);
  print( test4, "\n"); // Expected output: 1

  var test5 := Abs(9);
  print( test5, "\n"); // Expected output: 9
}


method Main()
{
  TestAbs();
}

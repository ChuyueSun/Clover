method MultipleReturns(x: int, y: int) returns (more: int, less: int)
{
  if x < y {
    more := y;
    less := x;
  } else {
    more := x;
    less := y;
  }

}
method TestMethod() {
  var more, less := MultipleReturns(9,3);
  print("Test 1: MultipleReturns(9,3) = ", more, ",", less, "\n");

  more, less := MultipleReturns(100,25);
  print("Test 2: MultipleReturns(100,25) = ", more, ",", less, "\n");

  more, less:= MultipleReturns(-4,2);
  print("Test 3: MultipleReturns(-4,2) = ", more, ",", less, "\n");

  more, less:= MultipleReturns(10,-5);
  print("Test 4: MultipleReturns(10,-5) = ", more, ",", less, "\n");

  more, less:= MultipleReturns(0,0);
  print("Test 5: MultipleReturns(0,0) = ", more, ",", less, "\n");
}

method Main() {
  TestMethod();
}

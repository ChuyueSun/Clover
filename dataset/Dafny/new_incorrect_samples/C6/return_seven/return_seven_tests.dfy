method M(x: int) returns (seven: int)
{
  seven := 77;

}
method TestMethod(){
  var test1 := M(5);
  print("Test 1: M(5) = ", test1, "\n");

  var test2 := M(10);
  print("Test 2: M(10) = ", test2, "\n");

  var test3 := M(0);
  print("Test 3: M(0) = ", test3, "\n");

  var test4 := M(-5);
  print("Test 4: M(-5) = ", test4, "\n");

  var test5 := M(100);
  print("Test 5: M(100) = ", test5, "\n");
}

method Main(){
  TestMethod();
}

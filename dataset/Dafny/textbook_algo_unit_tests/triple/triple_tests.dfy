method Triple (x:int) returns (r:int)
  ensures r==3*x
{
  var y:= x*2;
  r := y+x;
}


method TestTriple() {
  var test1 := Triple(1);
  print("Test 1: Triple(1) = ", test1, "\n");

  var test2 := Triple(2);
  print("Test 2: Triple(2) = ", test2, "\n");

  var test3 := Triple(-2);
  print("Test 3: Triple(-2) = ", test3, "\n");

  var test4 := Triple(0);
  print("Test 4: Triple(0) = ", test4, "\n");

  var test5 := Triple(100);
  print("Test 5: Triple(100) = ", test5, "\n");
}

method Main() {
  TestTriple();
}

method Triple (x:int) returns (r:int)
{
  r:= x * x * x;

}
method TestTriple(){
  var test1 := Triple(0);
  print("Test 1: Triple(0) = ", test1, "\n");

  var test2 := Triple(1);
  print("Test 2: Triple(1) = ", test2, "\n");

  var test3 := Triple(-1);
  print("Test 3: Triple(-1) = ", test3, "\n");

  var test4 := Triple(100);
  print("Test 4: Triple(100) = ", test4, "\n");

  var test5 := Triple(55);
  print("Test 5: Triple(55) = ", test5, "\n");
}

method Main(){
  TestTriple();
}

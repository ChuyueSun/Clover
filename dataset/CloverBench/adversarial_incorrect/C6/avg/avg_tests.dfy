method ComputeAvg(a: int, b: int) returns (avg:int)
{
  avg:= a + b;

}
method TestComputeAvg(){
  var test1 := ComputeAvg(2, 2);
  print("Test 1: ComputeAvg(2, 2) = ", test1, "\n");

  var test2 := ComputeAvg(7, 3);
  print("Test 2: ComputeAvg(7, 3) = ", test2, "\n");

  var test3 := ComputeAvg(100, 200);
  print("Test 3: ComputeAvg(100, 200) = ", test3, "\n");

  var test4 := ComputeAvg(0, 0);
  print("Test 4: ComputeAvg(0, 0) = ", test4, "\n");

  var test5 := ComputeAvg(-7, -2);
  print("Test 5: ComputeAvg(-7, -2) = ", test5, "\n");
}

method Main(){
  TestComputeAvg();
}

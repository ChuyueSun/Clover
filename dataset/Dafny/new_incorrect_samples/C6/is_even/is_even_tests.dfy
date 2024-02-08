method ComputeIsEven(x:int) returns (is_even:bool)
{
  if x%2==0{
    is_even:=true;
  }

}
method TestComputeIsEven(){
  var test1 := ComputeIsEven(2);
  print("Test 1: ComputeIsEven(2) = ", test1, "\n");

  var test2 := ComputeIsEven(3);
  print("Test 2: ComputeIsEven(3) = ", test2, "\n");

  var test3 := ComputeIsEven(0);
  print("Test 3: ComputeIsEven(0) = ", test3, "\n");

  var test4 := ComputeIsEven(-1);
  print("Test 4: ComputeIsEven(-1) = ", test4, "\n");

  var test5 := ComputeIsEven(-2);
  print("Test 5: ComputeIsEven(-2) = ", test5, "\n");
}

method Main(){
  TestComputeIsEven();
}

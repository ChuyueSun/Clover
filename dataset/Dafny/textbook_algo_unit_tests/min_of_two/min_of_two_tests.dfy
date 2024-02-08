method Min(x: int, y:int) returns (z: int)
  ensures x<=y ==> z==x
  ensures x>y ==> z==y
{
  if x < y {
    return x;
  } else {
    return y;
  }
}

method TestMin(){
  var test1 := Min(7,10);
  print("Test 1: Min(7,10) = ", test1, "\n");

  var test2 := Min(20,5);
  print("Test 2: Min(20,5) = ", test2, "\n");

  var test3 := Min(-3,-6);
  print("Test 3: Min(-3,-6) = ", test3, "\n");

  var test4 := Min(100,100);
  print("Test 4: Min(100,100) = ", test4, "\n");

  var test5 := Min(0,22);
  print("Test 5: Min(0,22) = ", test5, "\n");
}

method Main(){
  TestMin();
}

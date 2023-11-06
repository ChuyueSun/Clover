method SquareRoot(N:nat) returns (r:nat)
  ensures r*r <= N < (r+1)*(r+1)
{
  r:=0;
  while (r+1)*(r+1)<=N
    invariant r*r<=N
  {
    r:=r+1;
  }
}

method TestSquareRoot(){
  var test1 := SquareRoot(4);
  print("Test 1: SquareRoot(4) = ", test1, "\n");

  var test2 := SquareRoot(10);
  print("Test 2: SquareRoot(10) = ", test2, "\n");

  var test3 := SquareRoot(0);
  print("Test 3: SquareRoot(0) = ", test3, "\n");

  var test4 := SquareRoot(15);
  print("Test 4: SquareRoot(15) = ", test4, "\n");

  var test5 := SquareRoot(25);
  print("Test 5: SquareRoot(25) = ", test5, "\n");
}

method Main(){
  TestSquareRoot();
}

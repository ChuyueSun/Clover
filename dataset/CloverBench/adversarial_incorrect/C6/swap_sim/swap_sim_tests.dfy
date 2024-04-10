method SwapSimultaneous(X: int, Y: int) returns(x: int, y: int)
{
  x, y := Y + X, Y - X;

}
method TestSwapSimultaneous(){
  var x, y := SwapSimultaneous(3, 5);
  print("Test 1: SwapSimultaneous(3,5) = ", x, y, "\n");

  x, y := SwapSimultaneous(-1, 0);
  print("Test 2: SwapSimultaneous(-1,0) = ",x, y, "\n");

  x, y := SwapSimultaneous(0, 0);
  print("Test 3: SwapSimultaneous(0,0) = ",x, y,  "\n");

  x, y := SwapSimultaneous(100, -100);
  print("Test 4: SwapSimultaneous(100,-100) = ", x, y,  "\n");

  x, y := SwapSimultaneous(123456789, 987654321);
  print("Test 5: SwapSimultaneous(123456789,987654321) = ", x, y,  "\n");
}

method Main(){
  TestSwapSimultaneous();
}


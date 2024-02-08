method SwapBitvectors(X: bv8, Y: bv8) returns(x: bv8, y: bv8)
  ensures x==Y
  ensures y==X
{
  x, y := X, Y;

  x := x ^ y;
  y := x ^ y;
  x := x ^ y;

}

method TestSwapBitvectors(){
  var x, y := SwapBitvectors(3, 5);
  print("Test 1: Swap(3,5) = ", x, y, "\n");

  x, y := SwapBitvectors(-1, 0);
  print("Test 2: Swap(-1,0) = ",x, y, "\n");

  x, y := SwapBitvectors(0, 0);
  print("Test 3: Swap(0,0) = ",x, y,  "\n");

  x, y := SwapBitvectors(100, -100);
  print("Test 4: Swap(100,-100) = ", x, y,  "\n");

  x, y := SwapBitvectors(189, 21);
  print("Test 5: Swap(189, 21) = ", x, y,  "\n");
}

method Main(){
  TestSwapBitvectors();
}

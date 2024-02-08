method SwapArithmetic(X: int, Y: int) returns(x: int, y: int)
  ensures x==Y
  ensures y==X

{
  x, y := X, Y;

  x := y - x;
  y := y - x;
  x := y + x;

}

method TestSwapArithmetic(){
  var x, y := SwapArithmetic(3, 5);
  print("Test 1: Swap(3,5) = ", x, y, "\n");

  x, y := SwapArithmetic(-1, 0);
  print("Test 2: Swap(-1,0) = ",x, y, "\n");

  x, y := SwapArithmetic(0, 0);
  print("Test 3: Swap(0,0) = ",x, y,  "\n");

  x, y := SwapArithmetic(100, -100);
  print("Test 4: Swap(100,-100) = ", x, y,  "\n");

  x, y := SwapArithmetic(123456789, 987654321);
  print("Test 5: Swap(123456789,987654321) = ", x, y,  "\n");
}

method Main(){
  TestSwapArithmetic();
}

method foo(X: bv8, Y: bv8) returns(x: bv8, y: bv8)
  requires Y==0
  ensures x==Y
{
  x, y := X, Y;

  x := x ^ y;
  y := x ^ y;
  x := x ^ y;

}

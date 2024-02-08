method Min(x: int, y:int) returns (z: int)
  requires x>=0 && y>=0
  ensures x>y ==> z==y
{
  if x < y {
    return x;
  } else {
    return y;
  }
}

method Abs(x: int) returns (y: int)
  ensures x>=0 ==> x==y
{
  if x < 0 {
    return -x;
  } else {
    return x;
  }
}

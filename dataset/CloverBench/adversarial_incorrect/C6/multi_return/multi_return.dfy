method MultipleReturns(x: int, y: int) returns (more: int, less: int)
  ensures x >= y ==> more == x && less == y
  ensures x <= y ==> more == y && less == x
{
  if x < y {
    more := y;
    less := x;
  } else {
    more := x;
    less := y;
  }
}

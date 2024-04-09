method MultipleReturns(x: int, y: int) returns (more: int, less: int)
{
  if x < y {
    more := y;
    less := x;
  } else {
    more := x;
    less := y;
  }
}

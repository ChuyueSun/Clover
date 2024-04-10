method foo(x: int, y: int) returns (more: int, less: int)
  ensures more == x+y
{
  more := x + y;
  less := x - y;
}

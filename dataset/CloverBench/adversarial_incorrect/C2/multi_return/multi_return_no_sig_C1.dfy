method foo(x: int, y: int) returns (more: int, less: int)
  requires x>=0
  ensures more == x+y
  ensures less == x-y
{
  more := x + y;
  less := x - y;
}

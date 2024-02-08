method foo(x: int) returns (a: int, b: int)
  requires x==0
  ensures a == 2 * x && b == 4 * x
  ensures a==0
{
  a := 2 * x;
  b := 2 * a;
}

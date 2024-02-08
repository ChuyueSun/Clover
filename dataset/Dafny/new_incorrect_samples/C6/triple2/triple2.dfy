method Triple (x:int) returns (r:int)
  requires x == 1
  ensures r==3*x
{
  r := 3;
}

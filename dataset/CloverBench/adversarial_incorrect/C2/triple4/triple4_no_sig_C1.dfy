method foo (x:int) returns (r:int)
  requires x>=0
  ensures r==3*x
  ensures r>=0
{
  var y:= x*2;
  r := y+x;
}

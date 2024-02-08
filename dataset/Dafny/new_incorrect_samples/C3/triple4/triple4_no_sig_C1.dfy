method foo (x:int) returns (r:int)
  ensures r==1 ==> r==3
{
  var y:= x*2;
  r := y+x;
}

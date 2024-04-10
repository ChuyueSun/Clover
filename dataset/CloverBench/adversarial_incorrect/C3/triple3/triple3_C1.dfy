method Triple (x:int) returns (r:int)
  requires x==0
  ensures r==x
{
  if x==0 {
    r:=0;
  }
  else{
    var y:=2*x;
    r:= x+y;
  }
}

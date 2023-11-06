method foo (x:int) returns (r:int)
  //TOFILL
{
  if {
    case x<18 =>
      var a,b := 2*x, 4*x;
      r:=(a+b)/2;
    case 0<=x =>
      var y:=2*x;
      r:= x+y;
  }
}

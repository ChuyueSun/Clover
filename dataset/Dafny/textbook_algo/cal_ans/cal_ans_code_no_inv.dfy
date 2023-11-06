method CalDiv() returns (x:int, y:int)
{

  x, y := 0, 191;
  while 7 <= y
  {
    x := x+1;
    y:=191-7*x;
  }

}

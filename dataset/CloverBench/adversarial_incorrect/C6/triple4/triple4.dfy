method Triple (x:int) returns (r:int)
  ensures x != 0 ==> r != x
{
  r := x + 3;
}

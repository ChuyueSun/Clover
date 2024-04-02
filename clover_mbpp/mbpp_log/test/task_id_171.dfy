method PentagonPerimeter(side: int) returns (perimeter: int)
  requires side > 0
  ensures perimeter == 5 * side
{
  perimeter := 5 * side;
}

method PentagonPerimeterTest(){
  var res1:=PentagonPerimeter(5);
  print(res1); print("\n");
               
  var res2:=PentagonPerimeter(10);
  print(res2); print("\n");
               
  var res3:=PentagonPerimeter(15);
  print(res3); print("\n");
               

}

method Main(){
  PentagonPerimeterTest();
}
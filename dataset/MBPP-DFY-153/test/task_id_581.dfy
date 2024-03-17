method SquarePyramidSurfaceArea(baseEdge: int, height: int) returns (area: int)
  requires baseEdge > 0
  requires height > 0
  ensures area == baseEdge * baseEdge + 2 * baseEdge * height
{
  area := baseEdge * baseEdge + 2 * baseEdge * height;
}

method SquarePyramidSurfaceAreaTest(){
  var res1:=SquarePyramidSurfaceArea(3,4);
  print(res1);print("\n");
              
  var res2:=SquarePyramidSurfaceArea(4,5);
  print(res2);print("\n");
              

  var res3:=SquarePyramidSurfaceArea(1,1);
  print(res3);print("\n");
              

}

method Main(){
  SquarePyramidSurfaceAreaTest();
}

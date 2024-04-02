method CylinderLateralSurfaceArea(radius: real, height: real) returns (area: real)
  requires radius > 0.0 && height > 0.0
  ensures area == 2.0 * (radius * height) * 3.14159265358979323846
{
  area := 2.0 * (radius * height) * 3.14159265358979323846;
}

method CylinderLateralSurfaceAreaTest(){
  var res1:= CylinderLateralSurfaceArea(10.0,5.0);
  print(res1);print("\n");
              
  var res2:= CylinderLateralSurfaceArea(4.0,5.0);
  print(res2);print("\n");
              
  var res3:= CylinderLateralSurfaceArea(4.0,10.0);
  print(res3);print("\n");
              


}

method Main(){
  CylinderLateralSurfaceAreaTest();
}
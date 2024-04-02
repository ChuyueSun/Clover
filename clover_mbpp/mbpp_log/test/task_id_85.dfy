method SphereSurfaceArea(radius: real) returns (area: real)
  requires radius > 0.0
  ensures area == 4.0 * 3.14159265358979323846 * radius * radius
{
  area := 4.0 * 3.14159265358979323846 * radius * radius;
}

method SphereSurfaceAreaTest(){

  var res1:=SphereSurfaceArea(10.0);
  print(res1); print("\n");
               
  var res2:=SphereSurfaceArea(15.0);
  print(res2); print("\n");
               
  var res3:=SphereSurfaceArea(20.0);
  print(res3); print("\n");
               


}

method Main(){
  SphereSurfaceAreaTest();
}
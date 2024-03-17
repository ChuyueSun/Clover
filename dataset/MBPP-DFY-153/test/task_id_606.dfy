method DegreesToRadians(degrees: real) returns (radians: real)
  ensures radians == degrees * 3.14159265358979323846 / 180.0
{
  radians := degrees * 3.14159265358979323846 / 180.0;
}

method DegreesToRadiansTest(){
  var res1:=DegreesToRadians(90.0);
  print(res1);print("\n");
              

  var res2:=DegreesToRadians(60.0);
  print(res2);print("\n");
              

  var res3:=DegreesToRadians(120.0);
  print(res3);print("\n");
              

}

method Main(){
  DegreesToRadiansTest();
}

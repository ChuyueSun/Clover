method CubeVolume(size: int) returns (volume: int)
  requires size > 0
  ensures volume == size * size * size
{
  volume := size * size * size;
}


method CubeVolumeTest(){
  var out1:=CubeVolume(3);
  print(out1);print("\n");
              
  var out2:=CubeVolume(2);
  print(out2);print("\n");
              
  var out3:=CubeVolume(5);
  print(out3);print("\n");
              

}

method Main(){
  CubeVolumeTest();
}
method DogYears(humanYears: int) returns (dogYears: int)
  requires humanYears >= 0
  ensures dogYears == 7 * humanYears
{
  dogYears := 7 * humanYears;
}

method DogYearsTest(){
  var res1:=DogYears(12);
  print(res1);print("\n");
              
  var res2:=DogYears(15);
  print(res2);print("\n");
              
  var res3:=DogYears(24);
  print(res3);print("\n");
              
}

method Main(){
  DogYearsTest();
}
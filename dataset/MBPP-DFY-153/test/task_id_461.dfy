predicate IsUpperCase(c: char)
{
  65 <= c as int <= 90
}

method CountUppercase(s: string) returns (count: int)
  ensures count >= 0
  ensures count == | set i: int | 0 <= i < |s| && IsUpperCase(s[i])|
{
  var uppercase := set i: int | 0 <= i < |s| && IsUpperCase(s[i]);
  count := |uppercase|;
}

method CountUppercaseTest(){
  var out1:=CountUppercase("PYthon");
  print(out1);print("\n");
              
  var out2:=CountUppercase("BigData");
  print(out2);print("\n");
              
  var out3:=CountUppercase("program");
  print(out3);print("\n");
              

}

method Main(){
  CountUppercaseTest();
}

function countTo( a:array<bool>, n:int ) : int
  requires 0 <= n && n <= a.Length
  decreases n
  reads a
{
  if (n == 0) then 0 else countTo(a, n-1) + (if a[n-1] then 1 else 0)
}

method CountTrue(a: array<bool>) returns (result: int)
  ensures result == countTo(a, a.Length)
{
  result := 0;
  for i := 0 to a.Length
    invariant 0 <= i <= a.Length
    invariant result == countTo(a, i)
  {
    if a[i] {
      result := result + 1;
    }
  }
}

method CountTrueTest(){
  var a1:= new bool[] [true, false, true];
  var out1:=CountTrue(a1);
  print(out1);print("\n");
              
  var a2:= new bool[] [false, false];
  var out2:=CountTrue(a2);
  print(out2);print("\n");
              
  var a3:= new bool[] [true, true, true];
  var out3:=CountTrue(a3);
  print(out3);print("\n");
              


}

method Main(){
  CountTrueTest();
}
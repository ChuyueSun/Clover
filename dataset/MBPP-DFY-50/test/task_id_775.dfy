predicate IsOdd(n: int)
{
  n % 2 == 1
}

method IsOddAtIndexOdd(a: array<int>) returns (result: bool)
  ensures result <==> forall i :: 0 <= i < a.Length ==> (IsOdd(i) ==> IsOdd(a[i]))
{
  result := true;
  for i := 0 to a.Length
    invariant 0 <= i <= a.Length
    invariant result <==> forall k :: 0 <= k < i ==> (IsOdd(k) ==> IsOdd(a[k]))
  {
    if IsOdd(i) && !IsOdd(a[i])
    {
      result := false;
      break;
    }
  }
}

method IsOddAtIndexOddTest(){
  var a1:= new int[] [2,1,4,3,6,7,6,3];
  var out1:=IsOddAtIndexOdd(a1);
  print(out1);print("\n");
              

  var a2:= new int[] [4,1,2];
  var out2:=IsOddAtIndexOdd(a2);
  print(out2);print("\n");
              

  var a3:= new int[] [1,2,3];
  var out3:=IsOddAtIndexOdd(a3);
  print(out3);print("\n");
              

}

method Main(){
  IsOddAtIndexOddTest();
}

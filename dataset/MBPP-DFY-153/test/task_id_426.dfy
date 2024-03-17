/**
  * Filter odd numbers from an array of numbers
  **/

predicate IsOdd(n: int)
{
  n % 2 != 0
}

method FilterOddNumbers(arr: array<int>) returns (oddList: seq<int>)
  
  ensures forall i :: 0 <= i < |oddList| ==> IsOdd(oddList[i]) && oddList[i] in arr[..]
  
  ensures forall i :: 0 <= i < arr.Length && IsOdd(arr[i]) ==> arr[i] in oddList
{
  oddList := [];
  for i := 0 to arr.Length
    invariant 0 <= i <= arr.Length
    invariant 0 <= |oddList| <= i
    invariant forall k :: 0 <= k < |oddList| ==> IsOdd(oddList[k]) && oddList[k] in arr[..]
    invariant forall k :: 0 <= k < i && IsOdd(arr[k]) ==> arr[k] in oddList
  {
    if IsOdd(arr[i])
    {
      oddList := oddList + [arr[i]];
    }
  }
}

method FilterOddNumbersTest(){

  var a1:= new int[] [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  var res1:=FilterOddNumbers(a1);
  print(res1);print("\n");
              

  var a2:= new int[] [10,20,45,67,84,93];
  var res2:=FilterOddNumbers(a2);
  print(res2);print("\n");
              

  var a3:= new int[] [5,7,9,8,6,4,3];
  var res3:=FilterOddNumbers(a3);
  print(res3);print("\n");
              

}

method Main(){
  FilterOddNumbersTest();
}

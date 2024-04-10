method IsPalindrome(x: seq<char>) returns (result: bool)
  ensures result <==> (forall i :: 0 <= i < |x| ==> x[i] == x[|x| - i - 1])
{
  if |x|==0 {
    return true;
  }
  var i := 0;
  var j := |x| - 1;
  result := true;
  while (i < j)
    invariant 0<=i<=j+1 && 0<=j < |x|
    invariant i+j==|x|-1
    invariant (forall k :: 0 <= k < i ==> x[k] == x[|x| - k - 1])
  {
    if x[i] != x[j] {
      result := false;
      return;
    }
    i := i + 1;
    j := j - 1;
  }
}

method TestComputeIsEven(){
  var x:= "hello";
  var test1 := IsPalindrome(x);
  print("Test 1: ComputeIsEven(2) = ", test1, "\n");
  x:= "h0000oo0000h";
  var test2 := IsPalindrome(x);
  print("Test 2: ComputeIsEven(3) = ", test2, "\n");
  x:= "not";
  var test3 := IsPalindrome(x);
  print("Test 3: ComputeIsEven(0) = ", test3, "\n");
  x:= "yessey";
  var test4 := IsPalindrome(x);
  print("Test 4: ComputeIsEven(-1) = ", test4, "\n");
  x:= "hhhhhhhh";
  var test5 := IsPalindrome(x);
  print("Test 5: ComputeIsEven(-2) = ", test5, "\n");
}

method Main(){
  TestComputeIsEven();
}

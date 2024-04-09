method Match(s: string, p: string) returns (b: bool)
  requires |s| == |p|
  ensures b <==> forall n :: 0 <= n < |s| ==> s[n] == p[n] || p[n] == '?'
{
  var i := 0;
  while i < |s|
    invariant 0 <= i <= |s|
    invariant forall n :: 0 <= n < i ==> s[n] == p[n] || p[n] == '?'
  {
    if s[i] != p[i] && p[i] != '?'
    {
      return false;
    }
    i := i + 1;
  }
  return true;
}

method TestMatchMethod(){
  var test1 := Match("hello", "hello");
  print("Test 1: Match(\"hello\", \"hello\") = ", test1, "\n"); // Expected: true

  var test2 := Match("hello", "h?llo");
  print("Test 2: Match(\"hello\", \"h?llo\") = ", test2, "\n"); // Expected: true

  var test3 := Match("test", "t??t");
  print("Test 3: Match(\"test\", \"t??t\") = ", test3, "\n"); // Expected: true

  var test4 := Match("abcd", "ab?d");
  print("Test 4: Match(\"abcd\", \"ab?d\") = ", test4, "\n"); // Expected: true

  var test5 := Match("main", "m?in");
  print("Test 5: Match(\"main\", \"m?in\") = ", test5, "\n"); // Expected: false, since 'a' != 'i'
}

method Main(){
  TestMatchMethod();
}

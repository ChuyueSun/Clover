method LongestCommonPrefix(str1: seq<char>, str2: seq<char>) returns (prefix: seq<char>)
  ensures |prefix| <= |str1| && prefix == str1[0..|prefix|]&& |prefix| <= |str2| && prefix == str2[0..|prefix|]
  ensures |prefix|==|str1| || |prefix|==|str2| || (str1[|prefix|]!=str2[|prefix|])
{
  prefix := [];
  var minLength := if |str1| <|str2| then |str1| else |str2|;

  for idx:= 0 to minLength
    invariant |prefix|==idx <=  minLength<=|str1| && minLength<=|str2|
    invariant |prefix| <= |str1| && prefix == str1[0..|prefix|]&& |prefix| <= |str2| && prefix == str2[0..|prefix|]
  {
    if str1[idx] != str2[idx] {
      return;
    }
    prefix := prefix + [str1[idx]];
  }
}

method TestMethod(){
  // Test 1: Two identical sequences
  var test1_str1 := ['a', 'b', 'c', 'd'];
  var test1_str2 := ['a', 'b', 'c', 'd'];
  var test1_result := LongestCommonPrefix(test1_str1, test1_str2);
  print("Test 1: ", test1_result, "\n"); // Expected output: ['a', 'b', 'c', 'd']

  // Test 2: Two entirely distinct sequences
  var test2_str1 := ['a', 'b', 'c'];
  var test2_str2 := ['x', 'y', 'z'];
  var test2_result := LongestCommonPrefix(test2_str1, test2_str2);
  print("Test 2: ", test2_result, "\n"); // Expected output: []

  // Test 3: One sequence is the prefix of the other
  var test3_str1 := ['a', 'b', 'c'];
  var test3_str2 := ['a', 'b', 'c', 'd', 'e'];
  var test3_result := LongestCommonPrefix(test3_str1, test3_str2);
  print("Test 3: ", test3_result, "\n"); // Expected output: ['a', 'b', 'c']

  // Test 4: One sequence is empty
  var test4_str1 := [];
  var test4_str2 := ['x', 'y', 'z'];
  var test4_result := LongestCommonPrefix(test4_str1, test4_str2);
  print("Test 4: ", test4_result, "\n"); // Expected output: []

  // Test 5: Common prefix in both but not entire sequence
  var test5_str1 := ['a', 'b', 'x', 'y'];
  var test5_str2 := ['a', 'b', 'c', 'd'];
  var test5_result := LongestCommonPrefix(test5_str1, test5_str2);
  print("Test 5: ", test5_result, "\n"); // Expected output: ['a', 'b']
}

method Main(){
  TestMethod();
}

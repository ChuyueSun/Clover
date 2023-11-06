method SetToSeq<T>(s: set<T>) returns (xs: seq<T>)
  ensures multiset(s) == multiset(xs)
{
  xs := [];
  var left: set<T> := s;
  while left != {}
    invariant multiset(left) + multiset(xs) == multiset(s)
  {
    var x :| x in left;
    left := left - {x};
    xs := xs + [x];
  }
}

method TestSetToSeq(){

  var test1 := SetToSeq({1, 2, 3, 4, 5});
  print("Test 1: SetToSeq({1, 2, 3, 4, 5}) = ", test1, "\n");

  var test2 := SetToSeq({10, 20, 30, 40});
  print("Test 2: SetToSeq({10, 20, 30, 40}) = ", test2, "\n");

  var test3 := SetToSeq({100, 200, 300});
  print("Test 3: SetToSeq({100, 200, 300}) = ", test3, "\n");

  var test4 := SetToSeq({1000, 2000, 3000, 4000});
  print("Test 4: SetToSeq({1000, 2000, 3000, 4000}) = ", test4, "\n");

  var test5 := SetToSeq({10000, 20000, 30000, 40000, 50000});
  print("Test 5: SetToSeq({10000, 20000, 30000, 40000, 50000}) = ", test5, "\n");
}

method Main(){
  TestSetToSeq();
}

method CountLessThan(numbers: set<int>, threshold: int) returns (count: int)
  ensures count == |set i | i in numbers && i < threshold|
{
  count := 0;
  var shrink := numbers;
  var grow := {};
  while |shrink | > 0
    decreases shrink
    invariant shrink + grow == numbers
    invariant grow !! shrink
    invariant count == |set i | i in grow && i < threshold|
  {
    var i: int :| i in shrink;
    shrink := shrink - {i};
    var grow' := grow+{i};
    assert (set i | i in grow' && i < threshold) ==
           (set i | i in grow && i < threshold )+ if i < threshold then {i} else {};
    grow := grow + {i};
    if i < threshold {
      count := count + 1;
    }
  }
}

method TestMethod() {
  var s := set i | i in {1, 2, 3, 4, 5};
  var test1 := CountLessThan(s, 3);
  print("Test 1: CountLessThan(set{1, 2, 3, 4, 5}, 3) = ", test1, "\n");
  s := set i | i in {10, 20, 30, 40, 50};
  var test2 := CountLessThan(s, 25);
  print("Test 2: CountLessThan(set{10, 20, 30, 40, 50}, 25) = ", test2, "\n");
  s := set i | i in {5, 15, 25, 35, 45};
  var test3 := CountLessThan(s, 30);
  print("Test 3: CountLessThan(set{5, 15, 25, 35, 45}, 30) = ", test3, "\n");
  s := set i | i in {};
  var test4 := CountLessThan(s, 3);
  print("Test 4: CountLessThan(set{}, 5) = ", test4, "\n");
  s := set i | i in {100, 200, 300, 400, 500};
  var test5 := CountLessThan(s, 600);
  print("Test 5: CountLessThan(set{100, 200, 300, 400, 500}, 600) = ", test5, "\n");
}

method Main() {
  TestMethod();
}

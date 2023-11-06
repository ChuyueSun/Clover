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

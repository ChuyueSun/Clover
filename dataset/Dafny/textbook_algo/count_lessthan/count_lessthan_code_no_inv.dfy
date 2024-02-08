method CountLessThan(numbers: set<int>, threshold: int) returns (count: int)
{
  count := 0;
  var shrink := numbers;
  var grow := {};
  while |shrink | > 0
  {
    var i: int :| i in shrink;
    shrink := shrink - {i};
    var grow' := grow+{i};
    grow := grow + {i};
    if i < threshold {
      count := count + 1;
    }
  }

}

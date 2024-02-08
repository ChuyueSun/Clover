method has_close_elements(numbers: seq<real>, threshold: real) returns (res: bool)
{
  res := false;
  var idx: int := 0;
  while idx < |numbers| && !res

  {
    var idx2: int := 0;
    while idx2 < idx && !res
       {

      var distance :=  (if numbers[idx2] - numbers[idx] < 0.0 then numbers[idx] - numbers[idx2] else numbers[idx2] - numbers[idx]);
      if distance < threshold  {
        res := true;
        return;
      }

      idx2 := idx2 + 1;
    }
    idx := idx + 1;
  }
}


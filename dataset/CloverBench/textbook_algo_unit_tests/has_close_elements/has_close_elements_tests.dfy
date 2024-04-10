method has_close_elements(numbers: seq<real>, threshold: real) returns (res: bool)
  ensures res ==> exists i: int, j: int :: 0 <= i < |numbers| && 0 <= j < |numbers| && i != j && (if numbers[i] - numbers[j] < 0.0 then numbers[j] - numbers[i] else numbers[i] - numbers[j]) < threshold
  ensures !res ==> (forall i: int, j: int :: 1 <= i < |numbers| && 0 <= j < i ==>  (if numbers[i] - numbers[j] < 0.0 then numbers[j] - numbers[i] else numbers[i] - numbers[j]) >= threshold)
{

  res := false;
  var idx: int := 0;
  while idx < |numbers| && !res
    invariant 0 <= idx <= |numbers|
    invariant !res
    invariant forall i: int, j: int :: 0 <= i < idx && 0 <= j < i ==> (if numbers[i] - numbers[j] < 0.0 then numbers[j] - numbers[i] else numbers[i] - numbers[j]) >= threshold
  {
    var idx2: int := 0;
    while idx2 < idx && !res
      invariant 0 <= idx <= |numbers|
      invariant 0 <= idx2 <= idx
      invariant !res
      invariant forall j: int :: 0 <= j < idx2 ==> (if numbers[idx] - numbers[j] < 0.0 then numbers[j] - numbers[idx] else numbers[idx] - numbers[j]) >= threshold
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


method TestHasCloseElements()
{
  var numbers:= [1.0, 2.0, 3.9, 4.0, 5.0, 2.2];
  var threshold:=0.3;
  var test := has_close_elements(numbers, threshold);
  print( test, "\n");

  numbers:= [1.0, 2.0, 3.9, 4.0, 5.0, 2.2];
  threshold:=0.05;
  test := has_close_elements(numbers, threshold);
  print(test, "\n");

  numbers:= [1.0, 2.0, 5.9, 4.0, 5.0];
  threshold:=0.95;
  test := has_close_elements(numbers, threshold);
  print(test, "\n");

  numbers:= [1.0, 2.0, 3.0, 4.0, 5.0, 2.0];
  threshold:=0.1;
  test := has_close_elements(numbers, threshold);
  print( test, "\n");

  numbers:= [1.1, 2.2, 3.1, 4.1, 5.1];
  threshold:=1.0;
  test := has_close_elements(numbers, threshold);
  print( test, "\n");
}


method Main()
{
  TestHasCloseElements();
}

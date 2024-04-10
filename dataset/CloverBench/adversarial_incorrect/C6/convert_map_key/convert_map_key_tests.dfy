method convert_map_key(inputs: map<nat, bool>, f: nat->nat) returns(r:map<nat, bool>)
{
  r:= map k | k in inputs :: f(f(k)) := inputs[k];

}
method TestConvertMapKey(){
  var inputs1 := map[1:= true, 2:= false, 3:= true];
  var func1 := (x: nat) => x + 1;
  var test1 := convert_map_key(inputs1, func1);
  print("Test 1: convert_map_key(inputs1, func1) = ", test1, "\n");

  var inputs2 := map[4:= false, 5:= false];
  var func2 := (x: nat) => x * 2;
  var test2 := convert_map_key(inputs2, func2);
  print("Test 2: convert_map_key(inputs2, func2) = ", test2, "\n");

  var inputs3 := map[6:= true, 7:= false, 8:= true];
  var func3 := (x: nat) => x + 10;
  var test3 := convert_map_key(inputs3, func3);
  print("Test 3: convert_map_key(inputs3, func3) = ", test3, "\n");

  var inputs4 := map[9:= true, 10:= false];
  var func4 := (x: nat) => x*3 + 2;
  var test4 := convert_map_key(inputs4, func4);
  print("Test 4: convert_map_key(inputs4, func4) = ", test4, "\n");

  var inputs5 := map[11:= true, 12:= false, 13:= true];
  var func5 := (x: nat) => x*2 + 9;
  var test5 := convert_map_key(inputs5, func5);
  print("Test 5: convert_map_key(inputs5, func5) = ", test5, "\n");
}
method Main(){
  TestConvertMapKey();
}

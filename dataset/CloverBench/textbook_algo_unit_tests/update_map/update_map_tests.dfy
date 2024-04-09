method update_map<K(!new), V>(m1: map<K, V>, m2: map<K, V>) returns (r: map<K, V>)
  ensures (forall k :: k in m2 ==> k in r)
  ensures (forall k :: k in m1 ==> k in r)
  ensures  (forall k :: k in m2 ==> r[k] == m2[k])
  ensures  (forall k :: !(k in m2) && k in m1 ==> r[k] == m1[k])
  ensures  (forall k :: !(k in m2) && !(k in m1) ==> !(k in r))
{
  r:= map k | k in (m1.Keys + m2.Keys) :: if k in m2 then m2[k] else m1[k];
}

method TestUpdateMapMethod(){
  var m1 := map["one":= 1, "two":= 2];
  var m2:= map ["three":= 3, "four":= 4];
  var test := update_map(m1, m2);
  print("Test 1: update_map(...) = ", test, "\n");
  m1 := map["red":= 1, "blue":= 2];
  m2:= map ["red":= 2, "yellow":= 4];
  test := update_map(m1, m2);
  print("Test 2: update_map(...) = ", test, "\n");
  m1 := map["apple":= 3, "banana":= 5];
  m2:= map ["apple":= 4, "cherry":= 6];
  test := update_map(m1, m2);
  print("Test 3: update_map(...) = ", test, "\n");
  m1 := map["cat":= 7, "dog":= 9];
  m2:= map ["mouse":= 11, "elephant":= 13];
  test := update_map(m1, m2);

  print("Test 4: update_map(...) = ", test, "\n");
  m1 := map["NYC":= 15, "London":= 17];
  m2:= map ["London":= 18, "Madrid":= 19];
  test := update_map(m1, m2);

  print("Test 5: update_map(...) = ", test, "\n");

}

method Main(){
  TestUpdateMapMethod();
}

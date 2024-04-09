method Quotient(x: nat, y:nat) returns (r:int, q:int)
{
  r:=x;
  q:=0;

}
method TestQuotient(){
  var r, q := Quotient(10, 2);
  print("Test 1: Quotient(10, 2) = ", r, q, "\n");

  r, q := Quotient(15, 5);
  print("Test 2: Quotient(15, 5) = ", r, q,  "\n");

  r, q := Quotient(100, 10);
  print("Test 3: Quotient(100, 10) = ",r, q,  "\n");

  r, q :=  Quotient(50, 3);
  print("Test 4: Quotient(50, 3) = ", r, q, "\n");

  r, q :=  Quotient(30, 4);
  print("Test 5: Quotient(30, 4) = ", r, q, "\n");
}

method Main(){
  TestQuotient();
}

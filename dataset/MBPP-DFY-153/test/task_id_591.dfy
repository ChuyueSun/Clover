method SwapFirstAndLast(a: array<int>)
  requires a != null && a.Length > 0
  modifies a
  ensures a[0] == old(a[a.Length - 1]) && a[a.Length - 1] == old(a[0])
  ensures forall k :: 1 <= k < a.Length - 1 ==> a[k] == old(a[k])
{
  var temp := a[0];
  a[0] := a[a.Length - 1];
  a[a.Length - 1] := temp;
}

method SwapFirstAndLastTest(){
  var a1:= new int[][12, 35, 9, 56, 24];
  
  var res1:=SwapFirstAndLast(a1);


  
  
  


  
  
  


}

method Main(){
  SwapFirstAndLastTest();
}


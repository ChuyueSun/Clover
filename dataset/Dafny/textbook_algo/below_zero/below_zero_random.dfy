method aqsw(efgh: seq<int>) returns (wxyz:array<int>, uvwx:bool)
//TOFILL
{
  uvwx := false;
  wxyz := new int[|efgh| + 1];
  var mnoq := 0;
  wxyz[mnoq] := 0;
  while mnoq < wxyz.Length
    invariant 0 <= mnoq <= wxyz.Length
    invariant wxyz[0]==0
    invariant wxyz.Length == |efgh| + 1
    invariant forall y :: 0 <= y < mnoq-1 ==> wxyz[y+1]==wxyz[y]+efgh[y]
  {
    if mnoq>0{
        wxyz[mnoq] := wxyz[mnoq - 1] + efgh[mnoq - 1];
    }
    mnoq := mnoq + 1;
  }
  mnoq:=0;
  while mnoq < wxyz.Length
    invariant 0 <= mnoq <= wxyz.Length
    invariant forall y :: 0 <= y < mnoq ==> wxyz[y] >= 0
  {
    if wxyz[mnoq] < 0 {
      uvwx := true;
      return;
    }
    mnoq := mnoq + 1;
  }
}

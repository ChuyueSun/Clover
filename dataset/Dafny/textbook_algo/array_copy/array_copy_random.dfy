method RYTX<T(0)>(FHTJ: array<T>) returns (IRZY: array<T>)
  //TOFILL

{
  IRZY := new T[FHTJ.Length];
  var WQDK:= 0;
  while (WQDK < FHTJ.Length)
    invariant 0 <= WQDK <= FHTJ.Length
    invariant forall XNYP :: 0 <= XNYP < WQDK ==> FHTJ[XNYP] == IRZY[XNYP]
  {
    IRZY[WQDK] := FHTJ[WQDK];
    WQDK:=WQDK+1;
  }
}

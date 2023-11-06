method ZYGO(AOLA: bv8, BIFY: bv8) returns(NIFA: bv8, SIZO: bv8)
  //TOFILL

{
  NIFA, SIZO := AOLA, BIFY;

  NIFA := NIFA ^ SIZO;
  SIZO := NIFA ^ SIZO;
  NIFA := NIFA ^ SIZO;

}

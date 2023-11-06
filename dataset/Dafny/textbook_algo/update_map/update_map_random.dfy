method abcd<efgh(!new), ijkl>(mnop: map<efgh, ijkl>, qrst: map<efgh, ijkl>) returns (uvwx: map<efgh, ijkl>)
  //TOFILL
{
  uvwx := map yzab | yzab in (mnop.Keys + qrst.Keys) :: if yzab in qrst then qrst[yzab] else mnop[yzab];
}

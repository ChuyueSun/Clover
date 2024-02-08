method foo<K(!new), V>(m1: map<K, V>, m2: map<K, V>) returns (r: map<K, V>)
  ensures (forall k :: k in m1 || k in m2 ==> k in r)
  ensures  (forall k :: k in m2 ==> r[k] == m2[k])
  ensures  (forall k :: !(k in m2) && k in m1 ==> r[k] == m1[k])
  ensures  (forall k :: !(k in m2) && !(k in m1) ==> !(k in r))
{
  //TOFILL
}

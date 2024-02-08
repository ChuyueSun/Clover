predicate pre_original<K(!new), V>(m1: map<K,V>,m2: map<K,V>,r: map<K,V>){
  true
}

predicate pre_gen<K(!new), V>(m1: map<K,V>,m2: map<K,V>,r: map<K,V>)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<K(!new), V>(m1: map<K,V>,m2: map<K,V>,r: map<K,V>)
  ensures pre_original(m1,m2,r ) <==> pre_gen(m1,m2,r )
{
}

ghost predicate post_original<K(!new), V>(m1: map<K,V>,m2: map<K,V>,r: map<K,V>)
  requires pre_original(m1,m2,r){
  (forall k :: k in m2 ==> k in r)
  && (forall k :: k in m1 ==> k in r)
  &&  (forall k :: k in m2 ==> r[k] == m2[k])
  &&  (forall k :: !(k in m2) && k in m1 ==> r[k] == m1[k])
}


ghost predicate post_gen<K(!new), V>(m1: map<K,V>,m2: map<K,V>,r: map<K,V>)
  requires pre_original(m1,m2,r){
  true // (#POST) && ... (#POST)
}

lemma post_eq<K(!new), V>(m1: map<K,V>,m2: map<K,V>,r: map<K,V>)
  requires pre_original(m1,m2,r )
  requires pre_gen(m1,m2,r )
  ensures post_original(m1,m2,r ) <==> post_gen(m1,m2,r )
{
}


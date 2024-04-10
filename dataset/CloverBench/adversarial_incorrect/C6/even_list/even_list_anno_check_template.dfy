predicate pre_original(arr: array<int>,evenNumbers: array<int>)
  reads arr, evenNumbers
{
  true
}

predicate pre_gen(arr: array<int>,evenNumbers: array<int>)
  reads arr, evenNumbers
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(arr: array<int>,evenNumbers: array<int>)
  ensures pre_original(arr,evenNumbers) <==> pre_gen(arr,evenNumbers )
{
}

predicate post_original(arr: array<int>,evenNumbers: array<int>)
  requires pre_original(arr,evenNumbers)
  reads arr, evenNumbers
{
  (forall x {:trigger (x%2) }:: x in arr[..] &&  (x%2==0)==> x in evenNumbers[..])
  && (forall x :: x !in arr[..] ==> x !in evenNumbers[..])
  && (forall k :: 0 <= k < evenNumbers.Length ==> evenNumbers[k] % 2 == 0)
  && (forall k, l :: 0 <= k < l < evenNumbers.Length ==>
                       exists n, m :: 0 <= n < m < arr.Length && evenNumbers[k] == arr[n] && evenNumbers[l] == arr[m])
}

predicate post_gen(arr: array<int>,evenNumbers: array<int>)
  requires pre_original(arr,evenNumbers)
  reads arr, evenNumbers
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(arr: array<int>,evenNumbers: array<int>)
  requires pre_original(arr,evenNumbers)
  requires pre_gen(arr,evenNumbers)
  ensures post_original(arr,evenNumbers) <==> post_gen(arr,evenNumbers)
{
}


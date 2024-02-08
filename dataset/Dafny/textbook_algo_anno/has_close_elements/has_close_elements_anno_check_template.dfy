predicate pre_original(numbers: seq<real>, threshold: real, res: bool)
{
  true
}

predicate pre_gen(numbers: seq<real>, threshold: real, res: bool)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(numbers: seq<real>, threshold: real, res: bool)
  ensures pre_original(numbers, threshold, res) <==> pre_gen(numbers, threshold, res )
{
}

predicate post_original(numbers: seq<real>, threshold: real, res: bool)
  requires pre_original(numbers, threshold, res)
{
  (res ==> exists i: int, j: int :: 0 <= i < |numbers| && 0 <= j < |numbers| && i != j && (if numbers[i] - numbers[j] < 0.0 then numbers[j] - numbers[i] else numbers[i] - numbers[j]) < threshold) &&
  (!res ==> (forall i: int, j: int :: 1 <= i < |numbers| && 0 <= j < i ==>  (if numbers[i] - numbers[j] < 0.0 then numbers[j] - numbers[i] else numbers[i] - numbers[j]) >= threshold))
}

predicate post_gen(numbers: seq<real>, threshold: real, res: bool)
  requires pre_original(numbers, threshold, res)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(numbers: seq<real>, threshold: real, res: bool)
  requires pre_original(numbers, threshold, res)
  requires pre_gen(numbers, threshold, res)
  ensures post_original(numbers, threshold, res) <==> post_gen(numbers, threshold, res)
{
}


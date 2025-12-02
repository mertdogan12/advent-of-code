let start = 50;;
let all = 100;;

let modulo x y =
  let res = x mod y in
  if res >= 0 then res
  else res + y;;

let clicks_r x y = (x + y) / 100;;

let clicks_l x y =
  let cl = clicks_r (modulo (x - y) 100) y in
  if x == 0 then max 0 (cl - 1)
  else if (modulo (x - y) 100) == 0 then (cl + 1)
  else cl;;

let parse_sec sec =
  (
    sec.[0],
    int_of_string (String.sub sec 1 (String.length sec - 1))
  )

let rec solve ic r out =
  try
    match parse_sec (input_line ic) with
    | ('L', x) -> solve ic (modulo (r - x) all) (out + clicks_l r x)
    | ('R', x) -> solve ic (modulo (r + x) all) (out + clicks_r r x)
    | (x, _) -> raise (Invalid_argument ("Invalid direction " ^ String.make 1 x))
  with End_of_file ->
    out;;

let solve ic = solve ic start 0;;

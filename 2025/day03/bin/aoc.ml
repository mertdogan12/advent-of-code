
let explode s =
  let rec expl i l =
    if i < 0 then l else
    expl (i - 1) (s.[i] :: l) in
  expl (String.length s - 1) [];;

let implode l =
  let rec imp s l =
    match l with
    | x :: y -> imp (s ^ string_of_int x) y
    | [] -> s in
  imp "" l;;

let sublist list i =
  let rec cut list out c i =
    if i < c then out
    else cut list (List.nth list i :: out) c (i - 1) in
  cut list [] i (List.length list - 1);;

let rec fst_highest inp c cpos pos rem =
  let (c, cpos) = match inp with
  | x :: y -> let i = Char.Ascii.digit_to_int x in
    if List.length y == rem then (c, cpos)
    else if i > c then
      fst_highest y i pos (pos + 1) rem
    else
      fst_highest y c cpos (pos + 1) rem
  | [] -> (c, cpos) in
  (c, cpos)

let highest inp =
  let rec h s_pos count list =
    if count > 11 then list
    else let (c, cpos) = fst_highest (sublist inp s_pos) 0 0 s_pos (10 - count) in
    h (cpos + 1) (count + 1) (c :: list) in
  h 0 0 [];;

let calc l = int_of_string (implode (List.rev l));;

let rec solve ic out =
  try
    let i = calc (highest (explode (input_line ic))) in
    solve ic (out + i)
  with End_of_file ->
    out;;

let solve ic = solve ic 0;;

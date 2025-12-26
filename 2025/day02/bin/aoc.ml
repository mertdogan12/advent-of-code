let valid x =
  let s = string_of_int x in
  let s_len = String.length s in
  if s_len mod 2 = 0 then
    (String.sub s 0 (s_len / 2)) = (String.sub s (s_len / 2) (s_len / 2))
  else
    false;;

let rec calc_sec lo hi =
  if lo > hi then
    0
  else
    calc_sec (lo + 1) hi + if valid lo then lo else 0

let calc_sec sec =
  match String.split_on_char '-' sec with
  | x :: y :: _ -> calc_sec (int_of_string x) (int_of_string y)
  | _ -> failwith sec

let rec out inp =
  match inp with
  | [] -> 0
  | x :: y -> calc_sec x + out y;;

let out inp = out (String.split_on_char ',' inp);;

let solve ic = out (input_line ic);;

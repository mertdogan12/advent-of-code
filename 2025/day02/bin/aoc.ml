let rec valid_seq x seq =
  let seq_l = String.length seq in
  let x_l = String.length x in
  if x = "" then
    true
  else if seq_l > x_l then
    false
  else
    String.sub x 0 seq_l = seq && valid_seq (String.sub x seq_l (x_l - seq_l)) seq;;

let rec valid s i =
  if i > String.length s / 2 then
    false
  else
    valid_seq s (String.sub s 0 i) || valid s (i + 1);;

let rec calc_sec lo hi =
  if lo > hi then
    0
  else
    calc_sec (lo + 1) hi + if valid (string_of_int lo) 1 then lo else 0

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

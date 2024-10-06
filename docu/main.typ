#import "template.typ": *

#show: project.with(
  title: "Gauß-Newton",
  authors: (
    "Bobitsmagic",
  ),
)

$ f(x, y) = sum_i^P x_i y_i $
$ E(x) = 1 / 2 sum_s^S (f(x, p_s) - t_s)^2 $
$ Delta E = mat((delta E) / (delta x_0); 
  (delta E) / (delta x_1); dots.v) $
$ Delta_E = mat(sum_s^S (f(x, p^s) - t^s) (p^s_0); 
  sum_s^S (f(x, p^s) - t^s) (p^s_1); dots.v) $
$  H_E = mat(sum (p_0^s)^2, sum p_0^s p_1^s, dots; 
             sum p_1^s p_0^s, sum (p_1^s)^2, dots; 
             dots.v, dots.v, dots.down) $

$ H_E v = Delta_E $

$ E(x) = sum_s ((sum_p x_p y_(s p)) - t_s)^2 $
$ 1/2 (f_s(x))^2 = ((sum_p x_p y_(s p)) - t_s)^2 $
$ f_s(x) = sqrt(2) ((sum_p x_p y_(s p)) - t_s) $

$ J = mat((delta f_0)/(delta x_0), (delta f_0) / (delta x_1), dots; 
  (delta f_1)/(delta x_0), (delta f_1) / (delta x_1), dots;
  dots.v, dots.v, dots.down) $

$ J = sqrt(2) mat(y_(00), y_(01), dots;
  y_10, y_11, dots; 
  dots.v, dots.v, dots.down) $

$ J^T J = mat(y_00^2) $

= Move generation
== Pseudo moves
Pawns: Bitboard parallel
Knights: Per knight 
Slider: Per slider 
  perfect hashing or bitmask sort or 
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

#pagebreak()
$
  vec(1, 0) mat(1, 0) = mat(1, 0; 0, 0) \
  vec(a, b) mat(c, d) = mat(a c, a d; b c, b d) \
  vec(a, b, c) -> mat(a, a, a; b, b, b; c, c, c) \
  vec(x, y, z) -> mat(x, y, z; x, y, z; x, y, z) \
  M = mat(a, a, a; b, b, b; c, c, c) + mat(x, y, z; x, y, z; x, y, z) \

  abs(abs(P - M))_2^2 \

  sum_(i, j) (P_(i j) - M_(i j))^2 \
  sum_(i, j) (P_(i j) - (c_i + r_j))^2 \
$

$
  partial / (partial c_k) sum_(i, j) (P_(i j) - (c_i + r_j))^2 
  &= partial / (partial c_k) sum_(j)^n (P_(k j) - (c_k + r_j))^2 \
  &=  sum_(j)^n 1/2 (P_(k j) - (c_k + r_j)) dot (-1) \
  &= -1 / 2 sum_(j)^n (P_(k j) - (c_k + r_j)) \
  &= -1 / 2 sum_(j)^n P_(k j) - (c_k + r_j) \
  &= -1 / 2 (-n c_k + sum_(j) P_(k j) - r_j) \
  &= 1 / 2 (n c_k - sum_(j) P_(k j) - r_j) \
  &-> n c_k + sum_j r_j = sum_(j) P_(k j) \
  &-> n r_k + sum_i c_i = sum_(i) P_(i k) \
$

== Example
$
  n = 2\
  P = mat(1,2;3,4) \
  vec(c_0, c_1), mat(r_0, r_1) \
  mat(
    n, 0, 1, 1; 
    0, n, 1, 1;
    1, 1, n, 0;
    1, 1, 0, n
  ) 
  vec(c_0, c_1, r_0, r_1) = vec(sum_j P_(0 j), sum_j P_(1 j), sum_i P_(i 0), sum_i P_(i 1) ) \
  mat(
    2, 0, 1, 1; 
    0, 2, 1, 1;
    1, 1, 2, 0;
    1, 1, 0, 2
  ) 
  vec(c_0, c_1, r_0, r_1) = vec(3, 7, 4, 6) \
  mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    1, 1, 2, 0, 4;
    1, 1, 0, 2, 6;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    2, 2, 4, 0, 8;
    2, 2, 0, 4, 12;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    0, 2, 3, -1, 5;
    0, 2, -1, 3, 9;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    0, 0, 2, -2, -2;
    0, 0, -2, 2, 2;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    0, 0, 2, -2, -2;
    0, 0, 0, 0, 0;
    augment: #4
  ) \
    mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    0, 0, 1, -1, -1;
    0, 0, 0, 0, 0;
    augment: #4
  ) \

  r_0 = r_1 -1 \
  2c_1 +r_0 + r_1 = 7 <=> c_1 = 4 - r_1 \
  2 c_0 +r_0 +r_1 = 3 <=> c_0 = 2 -r_1 \
$

$
  mat(
    1, 0, 1, 0; 
    1, 0, 0, 1;
    0, 1, 1, 0;
    0, 1, 0, 1
  ) 
  vec(c_0, c_1, r_0, r_1) = vec(1, 2, 3, 4) \
  mat(
    1, 0, 1, 0, 1; 
    1, 0, 0, 1, 2;
    0, 1, 1, 0, 3;
    0, 1, 0, 1, 4; 
    augment: #4
  ) \
  mat(
    1, 0, 1, 0, 1; 
    0, 1, 1, 0, 3;
    0, 0, -1, 1, 1;
    0, 1, 0, 1, 4; 
    augment: #4
  ) \

  mat(
    1, 0, 1, 0, 1; 
    0, 1, 1, 0, 3;
    0, 0, -1, 1, 1;
    0, 0, -1, 1, 1; 
    augment: #4
  ) \
    mat(
    1, 0, 1, 0, 1; 
    0, 1, 1, 0, 3;
    0, 0, -1, 1, 1;
    0, 0, 0, 0, 0; 
    augment: #4
  ) \

  r_1 - 1= r_0 \
  c_1 + r_0 = 3 <=> c_1 = 4 - r_1 \
  c_0 + r_0 = 1 <=> c_0 = 2 - r_1 \

  mat(-1, 0) + vec(2, 4)
$

$
  mat(
    1, 0, 1, 0; 
    1, 0, 0, 1;
    0, 1, 1, 0;
    0, 1, 0, 1
  ) 
  vec(c_0, c_1, r_0, r_1) = vec(1, 2, 3, 4) \
  mat(
    1, 0, 1, 0, a; 
    1, 0, 0, 1, b;
    0, 1, 1, 0, c;
    0, 1, 0, 1, d; 
    augment: #4
  ) \
  mat(
    1, 0, 1, 0, a; 
    0, 1, 1, 0, c;
    0, 0, -1, 1, b-a;
    0, 1, 0, 1, d; 
    augment: #4
  ) \

  mat(
    1, 0, 1, 0, a; 
    0, 1, 1, 0, c;
    0, 0, -1, 1, b-a;
    0, 0, -1, 1, d-c; 
    augment: #4
  ) \
    mat(
    1, 0, 1, 0, a; 
    0, 1, 1, 0, c;
    0, 0, -1, 1, b-a;
    0, 0, 0, 0, d-c-b+a; 
    augment: #4
  ) \

  mat(
    1, 0, 1, 0, a; 
    0, 1, 1, 0, c;
    0, 0, -1, 1, b-a;
    0, 0, 0, 0, d-c-b+a; 
    augment: #4
  ) \

  r_1 - 1= r_0 \
  c_1 + r_0 = 3 <=> c_1 = 4 - r_1 \
  c_0 + r_0 = 1 <=> c_0 = 2 - r_1 \

  mat(-1, 0) + vec(2, 4)
$

$
  n = 2\
  P = mat(a, b; c,d) \
  vec(c_0, c_1), mat(r_0, r_1) \
  mat(
    n, 0, 1, 1; 
    0, n, 1, 1;
    1, 1, n, 0;
    1, 1, 0, n
  ) 
  vec(c_0, c_1, r_0, r_1) = vec(sum_j P_(0 j), sum_j P_(1 j), sum_i P_(i 0), sum_i P_(i 1) ) \
  mat(
    2, 0, 1, 1; 
    0, 2, 1, 1;
    1, 1, 2, 0;
    1, 1, 0, 2
  ) 
  vec(c_0, c_1, r_0, r_1) = vec(3, 7, 4, 6) \
  mat(
    2, 0, 1, 1, a + b; 
    0, 2, 1, 1, c + d;
    1, 1, 2, 0, a + c;
    1, 1, 0, 2, b + d;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, a + b; 
    0, 2, 1, 1, c + d;
    2, 2, 4, 0, 2a + 2c;
    2, 2, 0, 4, 2b + 2d;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, a + b; 
    0, 2, 1, 1, c + d;
    0, 2, 3, -1, a -b+2c;
    0, 2, -1, 3, -a + b + 2d;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, a + b; 
    0, 2, 1, 1, c + d;
    0, 0, 2, -2, a-b+c -d;
    0, 0, -2, 2, -a+b-c+d;
    augment: #4
  ) \
  mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    0, 0, 2, -2, -2;
    0, 0, 0, 0, 0;
    augment: #4
  ) \
    mat(
    2, 0, 1, 1, 3; 
    0, 2, 1, 1, 7;
    0, 0, 1, -1, -1;
    0, 0, 0, 0, 0;
    augment: #4
  ) \

  r_0 = r_1 + (a -b +c-d)/2 \
  2c_1 +r_0 + r_1 = c + d <=> c_1 = -(a -b + c - d) / 4 - r_1 + (c + d) / 2\
  2 c_0 + r_0 + r_1 = 3 <=> c_0 = -(a -b + c - d) / 4 - r_1 + (a + b) / 2\
$

#pagebreak()
$
  partial / (partial c_k) sum_(i, j) (P_(i j) - (c_i dot r_j))^2 
  &= partial / (partial c_k) sum_(j)^n (P_(k j) - (c_k dot r_j))^2 \
  &= sum_(j)^n 2 (P_(k j) - (c_k dot r_j)) dot (-r_j) \
  &= -2 sum_(j)^n (P_(k j) - (c_k dot r_j)) dot r_j \
  &= -2 sum_(j)^n P_(k j) r_j - c_k dot r_j^2 \
  &-> sum_(j)^n P_(k j) r_j - c_k sum_(j)^n r_j^2 \
$
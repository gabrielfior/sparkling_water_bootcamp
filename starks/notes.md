## Notes

Source: https://lambdaclass.github.io/lambdaworks/starks/recap.html

- In general, we express computation in our proving system by providing an *execution trace* satisfying certain constraints. The execution trace is a table containing the state of the system at every step of computation.
- Polynomials used to encode constraints (AIR)

### Fibonacci numbers

$a_0$ = 1 (1)

$a_1$ = 1 (2)

$a_{n+2}$ = $a_{n+1}$ + $a_{n}$ (3)

Execution trace for this simply a single column with all $a_i$ {1,1,2,3...}. It's valid if a. first 2 rows are 1 (boundary constraint) and b. value of any other row satisfies (3) above (transition constraint).

### Trace polynomial

Take the 8th primitive root of unity (because we have $2^3=8$ elements). Let trace polynomial be

$t(g^i) = a_i$

Our previous constraints translate to

- $t(g^0) = 1$ (a)
- $t(g^1) = 1$ (b)
- $t(x*g^2) - t(x*g) - t(x) = 0$ for all $x$ in ${g^0,g^1,...,g^5}$ -> Note that multiplying by g the same as advancing a row in the trace.

## Composition polynomial
Prover constructs another polynomial ($H$) that shows both boundary constraints (BC) and transition constraints (TC) are satisfied and commit to it, to prove to the verifier that trace polynomial satisfies relationships above.

### Boundary polynomial

Let P be a polynomial that interpolates the constraints (a) and (b), so P(1)=1, P(g)=1.

Boundary polynomial $B$ defined as

$B(x) = \frac{t(x) - P(x)}{(x01)(x-g)}$

where denominator is the boundary zerofier (or nullifier). B encodes boundary constraints because B has 1 and g as roots.

### Transition constraint polynomial

Similarly, transition constraint polynomial is C(x) 

$C(x) = \frac{t(x*g^2) - t(x*g) - t(x)}{ \prod_{i=0}^{5}(x - g^i)}$

C also vanishes at ${g^0, g^1, ..., g^5}$.

### Constructing H

We define H as

$H(x) = \beta_1B(x) + \beta_2C(x)$
Constants are there just to make it random and not predictable for the prover.

-> At this point, we use FRI as a black-box and commit to H.
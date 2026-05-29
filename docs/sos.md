# Structural Operational Semantics

## 1. Semantics Domains

### Machine State

$\sigma = \langle\rho_v, \rho_f\rangle$

$\rho_v$ is a stack of value scopes: Var $\rightarrow$ Val.

$\rho_f$ is a stack of function scopes: Var $\rightarrow$ FnDecl.

#### Values

$v ::=$ Lit(l) | UnInit | Mut(v)

#### Evaluation relation

$\sigma \vdash \text{expr} \Downarrow \text{v} , \sigma'$

$\sigma \vdash \text{stmt} \Downarrow \text{v} , \sigma'$

$\sigma \vdash \text{block} \Downarrow \text{v} , \sigma'$

---

## 2. Expressions

### 2.1 Literal

$$
\frac{
}{
  \sigma \vdash \mathrm{Lit}(l) \Downarrow \mathrm{Lit}(l),\ \sigma
}
\quad\text{[E-Lit]}
$$

---

### 2.2 Parenthesized expression

$$
\frac{
  \sigma \vdash e \Downarrow v,\ \sigma'
}{
  \sigma \vdash (e) \Downarrow v,\ \sigma'
}
\quad\text{[E-Par]}
$$

---

### 2.3 Identifier lookup

$$
\frac{
  \rho_v(x) = v
}{
  \sigma \vdash x \Downarrow v,\ \sigma
}
\quad\text{[E-Id]}
$$

---

### 2.4 Unary operators

#### Negation

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(n),\ \sigma'
}{
  \sigma \vdash -e \Downarrow \mathrm{Lit}(-n),\ \sigma'
}
\quad\text{[E-Neg]}
$$

#### Boolean negation

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(b),\ \sigma'
}{
  \sigma \vdash !e \Downarrow \mathrm{Lit}(\neg b),\ \sigma'
}
\quad\text{[E-Bang]}
$$

---

### 2.5 Binary operators

$$
\frac{
  \sigma \vdash e_1 \Downarrow v_1,\ \sigma_1
  \qquad
  \sigma_1 \vdash e_2 \Downarrow v_2,\ \sigma_2
  \qquad
  op(v_1,v_2) = v
}{
  \sigma \vdash e_1\ op\ e_2 \Downarrow v,\ \sigma_2
}
\quad\text{[E-BinOp]}
$$

---

### 2.6 If–then–else

#### True branch

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(\mathrm{true}),\ \sigma_1
  \qquad
  \sigma_1 \vdash B_t \Downarrow v,\ \sigma_2
}{
  \sigma \vdash \mathbf{if}\ e\ \{B_t\}\ \mathbf{else}\ \{B_f\}
  \Downarrow v,\ \sigma_2
}
\quad\text{[E-If-True]}
$$

#### False branch with else

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(\mathrm{false}),\ \sigma_1
  \qquad
  \sigma_1 \vdash B_f \Downarrow v,\ \sigma_2
}{
  \sigma \vdash \mathbf{if}\ e\ \{B_t\}\ \mathbf{else}\ \{B_f\}
  \Downarrow v,\ \sigma_2
}
\quad\text{[E-If-False]}
$$

#### False branch, no else

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(\mathrm{false}),\ \sigma'
}{
  \sigma \vdash \mathbf{if}\ e\ \{B_t\}
  \Downarrow \mathrm{Lit}(()),\ \sigma'
}
\quad\text{[E-If-Unit]}
$$

---

### 2.7 Block expression (no trailing semicolon)

$$
\frac{
  \sigma_{\text{push}}
  \vdash S_1 \Downarrow v_1,\ \sigma_1
  \quad\dots\quad
  \sigma_{n-1} \vdash S_n \Downarrow v_n,\ \sigma_n
}{
  \sigma \vdash \{S_1;...;S_n\} \Downarrow v_n,\ \mathrm{pop}(\sigma_n)
}
\quad\text{[E-Block-NoSemi]}
$$

Block with trailing semicolon:

$$
\frac{
  \sigma_{\text{push}}
  \vdash S_1 \Downarrow v_1,\ \sigma_1
  \quad\dots\quad
  \sigma_{n-1} \vdash S_n \Downarrow v_n,\ \sigma_n
}{
  \sigma \vdash \{S_1;...;S_n;\}
  \Downarrow \mathrm{Lit}(()),\ \mathrm{pop}(\sigma_n)
}
\quad\text{[E-Block-Semi]}
$$

---

### 2.8 Function calls

#### Built-in println

$$
\frac{
  \sigma \vdash args \Downarrow v^*,\ \sigma'
}{
  \sigma \vdash \mathrm{println!}(args)
  \Downarrow \mathrm{Lit}(()),\ \sigma'
}
\quad\text{[E-Call-Println]}
$$

#### User-defined function

$$
\frac{
  \rho_f(f)=\langle params,\ B\rangle
  \qquad
  \sigma \vdash args \Downarrow v_1..v_n,\ \sigma_1
  \qquad
  \sigma_2 = \mathrm{push}(\sigma_1 \cup \{params_i \mapsto v_i\})
  \qquad
  \sigma_2 \vdash B \Downarrow v,\ \sigma_3
}{
  \sigma \vdash f(args) \Downarrow v,\ \mathrm{pop}(\sigma_3)
}
\quad\text{[E-Call-Fn]}
$$

---

## 3. Statements

### 3.1 let with initializer

$$
\frac{
  \sigma \vdash e \Downarrow v,\ \sigma_1
}{
  \sigma \vdash \mathbf{let}\ x = e
  \Downarrow \mathrm{Lit}(()),\ \mathrm{define}(x\mapsto v,\sigma_1)
}
\quad\text{[S-Let-Init]}
$$

### 3.2 let without initializer

$$
\frac{
}{
  \sigma \vdash \mathbf{let}\ x
  \Downarrow \mathrm{Lit}(()),\ \mathrm{define}(x\mapsto \mathrm{UnInit},\sigma)
}
\quad\text{[S-Let-Uninit]}
$$

---

### 3.3 Assignment

$$
\frac{
  \sigma \vdash e \Downarrow v,\ \sigma_1
}{
  \sigma \vdash x = e
  \Downarrow \mathrm{Lit}(()),\ \mathrm{assign}(x:=v,\sigma_1)
}
\quad\text{[S-Assign]}
$$

---

### 3.4 While loop

#### Condition false

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(\mathrm{false}),\ \sigma_1
}{
  \sigma \vdash \mathbf{while}\ e\ \{B\}
  \Downarrow \mathrm{Lit}(()),\ \sigma_1
}
\quad\text{[S-While-False]}
$$

#### Condition true

$$
\frac{
  \sigma \vdash e \Downarrow \mathrm{Lit}(\mathrm{true}),\ \sigma_1
  \qquad
  \sigma_1 \vdash B \Downarrow v,\ \sigma_2
  \qquad
  \sigma_2 \vdash \mathbf{while}\ e\ \{B\}
        \Downarrow \mathrm{Lit}(()),\ \sigma_3
}{
  \sigma \vdash \mathbf{while}\ e\ \{B\}
  \Downarrow \mathrm{Lit}(()),\ \sigma_3
}
\quad\text{[S-While-True]}
$$

---

### 3.5 Function definition

$$
\frac{
}{
  \sigma \vdash \mathbf{fn}\ f(params)\{B\}
  \Downarrow \mathrm{Lit}(()),\ \mathrm{defineFn}(f,\sigma)
}
\quad\text{[S-Fn]}
$$

---

### 3.6 Expression statement

$$
\frac{
  \sigma \vdash e \Downarrow v,\ \sigma'
}{
  \sigma \vdash e \Downarrow v,\ \sigma'
}
\quad\text{[S-Expr]}
$$

---

## 4. Blocks (as statements)

Same rules as in Section 2.7.

---

## 5. Program Evaluation

$$
\frac{
  \sigma_0 = \mathrm{loadFns}(\mathrm{prog})
  \qquad
  \rho_f(\mathrm{main}) = \langle\emptyset,\ B\rangle
  \qquad
  \sigma_0 \vdash B \Downarrow v,\ \sigma'
}{
  \mathrm{prog} \Downarrow v
}
\quad\text{[P-Prog]}
$$

---

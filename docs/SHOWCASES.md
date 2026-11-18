# Bilingual showcase contracts

The examples index lists all twelve requested showcases and their source paths. Only the v0.1 examples are executable by this compiler. Each future example is a complete illustrative program under the assumptions below, not an accepted language specification or a tested implementation. Native-speaker review of identifiers and vocabulary remains a release requirement.

## v0.1 execution

```sh
mn run examples/hello.mn
mn run examples/hello_mn.mn
mn run examples/scalar.mn
mn run examples/scalar_mn.mn
mn run examples/functions.mn
mn run examples/functions_mn.mn
mn run examples/matmul.mn
mn run examples/matmul_mn.mn
mn run examples/mixed_syntax.mn
mn check examples/negative/shape_mismatch.mn --lang en
mn check examples/negative/shape_mismatch_mn.mn --lang mn
```

The last two commands intentionally return a nonzero exit status. Both operands' shapes must appear in the diagnostic, along with incompatible inner dimensions 3 and 2. Neither program is executed. Run those commands separately from a shell script that stops at the first nonzero exit.

## Proposed numerical API contracts

Broadcasting aligns dimensions from the right and expands singleton dimensions without changing numerical values. `[2,2] + [1,2]` produces `[2,2]`. A dimension of 1 broadcasts; unequal dimensions larger than 1 produce an error. The v0.1 checker instead requires exactly equal tensor shapes for elementwise operations.

`grad(square)` in the scalar example differentiates its sole scalar argument and returns a function from f64 to f64. At 3.0 the mathematical derivative is 6.0. This is an expected mathematical result, not a future compiler test result.

The two-layer MLP and optimizer showcase share one complete program per syntax. This avoids inconsistent initialization or training data across separate fragments. It includes deterministic data, explicit nonuniform weights, biases, a tanh hidden layer, mean squared scalar loss, native reverse gradients, SGD updates, 200 iterations, and loss reporting before every update and after training. Input `[4,2]` maps to hidden `[4,3]`, then output `[4,1]`. Bias shapes `[1,3]` and `[1,1]` broadcast across the batch. There is no randomness or external dataset.

Required future APIs and semantics:

| Construct | Proposed contract |
|---|---|
| `tensor::tanh` | Elementwise f64 tanh; derivative `1 - tanh(x)^2` |
| `tensor::mean` | Reduce all elements, return f64; derivative divides by element count |
| `grad(loss, argnums=0)` | Differentiate only the parameter struct; preserve its field tree and tensor shapes |
| `optim::SGD::new(0.05)` | Construct SGD with positive finite learning rate, no momentum or weight decay |
| `optimizer.step(&mut p, &gradient)` | Update every parameter element by `p = p - 0.05 * gradient`; no gradient accumulation |
| `&Parameters` | Shared borrow; `forward` and `loss` cannot mutate parameters |
| `&mut Parameters` | Exclusive borrow during optimizer mutation; no overlap with live AD tape reads |
| `0..200` | Exactly 200 iterations, indices 0 through 199 |
| `residual * residual` | Elementwise multiplication, not matrix multiplication |

The AD transform must finish using parameter values before the optimizer mutates them. The optimizer is outside the differentiated function. Borrow checking, tape storage, and effect analysis must enforce this sequencing. The training example is a design acceptance target; no final loss, convergence guarantee, throughput, or accuracy is claimed.

## Proposed device transfer

`to_device(cuda:0)` yields a recoverable result. `?` propagates unavailable-device, allocation, and transfer failures through a future fallible-entry-point convention. The entry-point convention itself needs an RFC before this syntax can become normative. `@` operates on colocated device values, preserves buffers until submitted work completes, and returns a device value. Moving to `cpu` waits for or explicitly carries the completion dependency before host printing. Missing CUDA hardware must fail explicitly rather than silently claim GPU execution.

## Proposed FFI companion

The future FFI examples require linking the following C translation unit. `double` must correspond to the target's C ABI f64 representation; supported targets must validate that mapping.

```c
/* square.c */
double mn_square(double x) {
    return x * x;
}
```

The foreign function has no pointer ownership transfer, callbacks, allocation, or unwinding. Calling it is explicitly unsafe because the compiler cannot verify the foreign implementation. There is no `mn` FFI build command today; platform-specific linking and ABI conformance tests are future work. Do not infer working static or dynamic linking from this example.

## Proposed imports

The import example uses a sibling `math.mn` module, whose one exported function is included as a companion-source comment in each example. For a future package named `demo`, the intended file layout is `mn.toml`, `main.mn`, and `math.mn`. A proposed manifest is:

```toml
[package]
name = "demo"
version = "0.1.0"
# edition intentionally omitted until governance selects its meaning

[dependencies]
```

The English companion exports `square`, and the Mongolian companion exports `квадрат`; each import matches its own symbol. Both compute `x * x`. Import resolution does not translate user-defined names across packages. Dependency resolution and lockfiles are distinct later features, not prerequisites for the local-module design example.

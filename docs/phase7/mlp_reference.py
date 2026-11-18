"""Independent reference for the proposed Mongolia MLP. Execute with Python 3."""
import json
import math

X = ((0., 0.), (0., 1.), (1., 0.), (1., 1.))
Y = (0., 1., 1., 2.)
# Row-major w1[2,3], b1[1,3], w2[3,1], b2[1,1].
INITIAL = (0.1, -0.2, 0.3, 0.2, 0.1, -0.1, 0., 0., 0., 0.2, -0.1, 0.3, 0.)


def evaluate(p):
    loss = 0.
    gradient = [0.] * len(p)
    outputs = []
    for x, target in zip(X, Y):
        hidden = [math.tanh(x[0]*p[h] + x[1]*p[3+h] + p[6+h]) for h in range(3)]
        prediction = sum(hidden[h]*p[9+h] for h in range(3)) + p[12]
        outputs.append(prediction)
        residual = prediction - target
        loss += residual ** 2 / len(Y)
        dy = 2. * residual / len(Y)
        gradient[12] += dy
        for h, activation in enumerate(hidden):
            gradient[9+h] += dy * activation
            dz = dy * p[9+h] * (1. - activation ** 2)
            gradient[6+h] += dz
            for i, value in enumerate(x):
                gradient[3*i+h] += value * dz
    return loss, gradient, outputs


def finite_difference(p, index, step):
    plus, minus = list(p), list(p)
    plus[index] += step
    minus[index] -= step
    return (evaluate(plus)[0] - evaluate(minus)[0]) / (2.*step)


def main():
    p = list(INITIAL)
    initial, grad, _ = evaluate(p)
    errors = {str(h): max(abs(g-finite_difference(p,i,h)) for i,g in enumerate(grad))
              for h in (1e-4, 1e-5, 1e-6)}
    if max(errors.values()) > 1e-7:
        raise ValueError("finite difference and analytic reference disagree")
    for _ in range(200):
        _, gradient, _ = evaluate(p)
        p = [v - 0.05*g for v,g in zip(p,gradient)]
    final, _, predictions = evaluate(p)
    if not (math.isfinite(final) and final < initial):
        raise ValueError("unexpected training trajectory")
    print(json.dumps(dict(implementation="Python analytic reference", initial_loss=initial,
                          gradient_at_initial=grad, max_abs_gradient_errors=errors,
                          loss_after_200_updates=final, predictions=predictions), indent=2))


if __name__ == "__main__":
    main()

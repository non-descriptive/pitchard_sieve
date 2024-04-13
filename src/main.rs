fn main() {
    let primes = sift(1000_000);
    println!("{}", primes.len());
}

#[allow(non_snake_case)]
struct Context {
    N: usize,
    max_s: usize,
    length: usize,
    p: usize,
    s: Vec<usize>,
}
fn sift(upto: usize) -> Vec<usize> {
    let capacity = (upto as f64).sqrt() as usize;
    let mut ctx = Context {
        N: upto,
        max_s: 1,
        length: 2,
        p: 3,
        s: Vec::with_capacity(capacity),
    };

    while ctx.p * ctx.p < upto {
        if ctx.length < upto {
            let n = upto.min(ctx.p * ctx.length);
            extend(&mut ctx, n);
        }
        delete_multiples(&mut ctx);
        ctx.p = next(&ctx, 1);
    }
    if ctx.length < ctx.N {
        let n = ctx.N;
        extend(&mut ctx, n);
    }
    collect_primes(&ctx, upto)
}
fn collect_primes(ctx: &Context, upto: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    ret.push(2);
    let mut p = 3;
    while p <= upto {
        ret.push(p);
        p = next(&ctx, p);
    }
    ret
}
fn delete(ctx: &mut Context, pf: usize) {
    let prev = prev(&ctx, pf);
    let next = next(&ctx, pf);
    ctx.s[prev - 1] = next;
    ctx.s[next - 2] = prev;
}
fn delete_multiples(mut ctx: &mut Context) {
    let mut f = ctx.p;
    while ctx.p * f <= ctx.length {
        f = next(&ctx, f);
    }
    while f > 1 {
        f = prev(&ctx, f);
        let pf = ctx.p * f;
        delete(&mut ctx, pf);
    }
}
fn extend(mut ctx: &mut Context, n: usize) {
    let sz = ctx.s.len();
    ctx.s.extend(std::iter::repeat(0).take(n - sz + 1));

    let mut w = 1;
    let mut x = ctx.length + 1;
    while x <= n {
        append(&mut ctx, x);
        w = next(&ctx, w);
        x = ctx.length + w;
    }
    ctx.length = n;
    if ctx.length == ctx.N {
        append(ctx, ctx.N + 2);
    }
}
fn append(ctx: &mut Context, wa: usize) {
    ctx.s[ctx.max_s - 1] = wa;
    ctx.s[wa - 2] = ctx.max_s;
    ctx.max_s = wa;
}
fn next(ctx: &Context, w: usize) -> usize {
    ctx.s[w - 1]
}
fn prev(ctx: &Context, w: usize) -> usize {
    ctx.s[w - 2]
}

fn concat_vecs<T>(vecs: Vec<Vec<T>>) -> Vec<T> {
    let size = vecs.iter().fold(0, |a, b| a + b.len());
    vecs.into_iter().fold(Vec::with_capacity(size), |mut acc, v| {
        acc.extend(v);
        acc
    })
}

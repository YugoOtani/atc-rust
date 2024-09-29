fn topological_sort<T: Eq + Hash>(ord: &Vec<(T, T)>) -> Vec<&T> {
    let mut cnt = HashMap::new();
    let mut to = HashMap::new();
    for (v1, v2) in ord {
        cnt.insert(v1, 0);
        cnt.insert(v2, 0);
        to.insert(v1, vec![]);
        to.insert(v2, vec![]);
    }
    for (v1, v2) in ord {
        *cnt.get_mut(v2).unwrap() += 1;
        to.get_mut(v1).unwrap().push(v2);
    }
    let mut ret = vec![];
    let mut q = VecDeque::new();
    for (k, v) in &cnt {
        if *v == 0 {
            q.push_back(*k);
        }
    }
    while let Some(v) = q.pop_front() {
        ret.push(v);
        for &vto in to.get(v).unwrap() {
            *cnt.get_mut(vto).unwrap() -= 1;
            if cnt.get(vto).unwrap() == &0 {
                q.push_back(vto);
            }
        }
    }
    ret
}
